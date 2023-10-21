use crate::{
    persistance::simulation::SimulationClient,
    schema::{
        electrolyzer::Electrolyzer,
        errors::{Error, Result},
        simulation_schema::{
            EmissionEvent, EnergySourcePortfolio, EnergyTransaction, HydrogenProductionEvent,
            PowerGrid, PowerPlant, SimulationResult, SimulationStatus, TaxCredit45V,
            TaxCredit45VTier, TaxCreditSummary,
        },
        time::{DateTimeRange, Timestamp},
        time_series::TimeSeriesChart,
    },
};
use chrono::{Datelike, Duration, Timelike};
use serde::{Deserialize, Serialize};

// https://ourworldindata.org/grapher/carbon-dioxide-emissions-factor
const COAL_MWH_TO_CO2: f64 = 353.88;
const NATURAL_GAS_MWH_TO_CO2: f64 = 201.96;
const PETROLEUM_MWH_TO_CO2: f64 = 266.76;
const BIOMASS_MWH_TO_CO2: f64 = 530.82;

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct SimulationState {
    pub id: i32,
    pub emissions: Vec<EmissionEvent>,
    pub hydrogen_productions: Vec<HydrogenProductionEvent>,
    pub transactions: Vec<EnergyTransaction>,
    pub tax_credit: Vec<TaxCredit45V>,
    pub tax_credit_summary: TaxCreditSummary,
}

impl SimulationState {
    fn new(simulation_id: i32) -> SimulationState {
        SimulationState {
            id: simulation_id,
            emissions: vec![],
            hydrogen_productions: vec![],
            transactions: vec![],
            tax_credit: vec![],
            tax_credit_summary: TaxCreditSummary::default(),
        }
    }
}

pub fn simulate(
    power_grid: &PowerGrid,
    electrolyzer: &Electrolyzer,
    time_range: &DateTimeRange,
    simulation_client: &Box<dyn SimulationClient>,
) -> Result<SimulationResult> {
    let simulation_id = simulation_client.get_next_id()?;
    let time_range = time_range.parse("%Y-%m-%dT%H:%M")?;
    let mut current_timestamp = time_range.start.to_utc_date_time()?;
    let mut end_timestamp = time_range.end.to_utc_date_time()?;
    let increment = Duration::minutes(15);
    let mut state = SimulationState::new(simulation_id);

    if current_timestamp.timestamp() > end_timestamp.timestamp() {
        return Err(Error::create_invalid_argument_error(
            "Simulation start must be before end time",
        ));
    }

    if current_timestamp.minute() % 15 != 0 {
        current_timestamp += Duration::minutes(15 - current_timestamp.minute() as i64 % 15);
    }

    if end_timestamp.minute() % 15 != 0 {
        end_timestamp += Duration::minutes(15 - end_timestamp.minute() as i64 % 15);
    }

    while current_timestamp < end_timestamp {
        let mut transactions = make_optimal_transactions(
            simulation_id,
            &Timestamp::from(current_timestamp),
            electrolyzer,
            power_grid,
        )?;
        let portfolio = create_energy_source_portfolio(&transactions);
        let emission_event = create_emission_event(
            simulation_id,
            &Timestamp::from(current_timestamp),
            electrolyzer,
            &portfolio,
        );
        let hydrogen_production_event = create_hydrogen_production_event(
            simulation_id,
            &Timestamp::from(current_timestamp),
            electrolyzer,
            &portfolio,
        )?;
        let tax_credit = calculate_tax_credit(&emission_event, &hydrogen_production_event);

        match tax_credit.tier {
            TaxCredit45VTier::Max => state.tax_credit_summary.credit_hours_full += 0.25,
            TaxCredit45VTier::Tier1 => state.tax_credit_summary.credit_hours_33 += 0.25,
            TaxCredit45VTier::Tier2 => state.tax_credit_summary.credit_hours_25 += 0.25,
            TaxCredit45VTier::Tier3 => state.tax_credit_summary.credit_hours_20 += 0.25,
            TaxCredit45VTier::None => state.tax_credit_summary.credit_hours_none += 0.25,
        }

        state.transactions.append(&mut transactions);
        state.emissions.push(emission_event);
        state.hydrogen_productions.push(hydrogen_production_event);
        state.tax_credit.push(tax_credit);

        current_timestamp += increment;
    }

    simulation_client.insert_simulation_state(&state)?;

    Ok(SimulationResult {
        status: SimulationStatus::Complete,
        tax_credit_summary: state.tax_credit_summary,
        emissions: produce_emissions_graph(&simulation_id)?,
        hydrogen_productions: TimeSeriesChart {
            id: String::from("hydrogen-produced"),
            title: String::from("Hydrogen Production over time"),
            data_set_endpoints: vec![format!("fetch_hydrogen_production/{simulation_id}")],
        },
        energy_costs: TimeSeriesChart {
            id: String::from("energy-costs"),
            title: String::from("Energy Costs Over Time"),
            data_set_endpoints: vec![format!("fetch_hydrogen_production/{simulation_id}")],
        },
    })
}

fn produce_emissions_graph(simulation_id: &i32) -> Result<TimeSeriesChart> {
    Ok(TimeSeriesChart {
        id: String::from("emissions"),
        title: String::from("Emissions over time"),
        data_set_endpoints: vec![format!("fetch_emissions/{simulation_id}")],
    })
}

fn make_optimal_transactions(
    simulation_id: i32,
    timestamp: &Timestamp,
    electrolyzer: &Electrolyzer,
    power_grid: &PowerGrid,
) -> Result<Vec<EnergyTransaction>> {
    power_grid
        .power_plants
        .iter()
        .map(|power_plant| purchase(simulation_id, electrolyzer, power_plant, 2.0, timestamp))
        .collect()
}

fn purchase(
    simulation_id: i32,
    electrolyzer: &Electrolyzer,
    power_plant: &PowerPlant,
    amount_mwh: f64,
    timestamp: &Timestamp,
) -> Result<EnergyTransaction> {
    let purchase_datetime = timestamp.to_utc_date_time()?;
    let generation = power_plant
        .generations
        .iter()
        .find(|generation| {
            generation.time_generated.to_utc_date_time().map_or_else(
                |_| false,
                |generation_datetime| {
                    purchase_datetime.year() == generation_datetime.year()
                        && purchase_datetime.month() == generation_datetime.month()
                        && purchase_datetime.day() == generation_datetime.day()
                        && purchase_datetime.hour() == generation_datetime.hour()
                },
            )
        })
        .ok_or_else(|| {
            Error::create_not_found_error(&format!("Generation not found for timestep"))
        })?;

    Ok(EnergyTransaction {
        simulation_id,
        electrolyzer_id: electrolyzer.id,
        plant_id: generation.plant_id,
        timestamp: timestamp.clone(),
        price_usd: generation.sale_price_usd_per_mwh * amount_mwh,
        portfolio: EnergySourcePortfolio::scale_to_amount(&generation.portfolio, amount_mwh)?,
    })
}

fn create_energy_source_portfolio(transactions: &Vec<EnergyTransaction>) -> EnergySourcePortfolio {
    transactions.iter().fold(
        EnergySourcePortfolio::default(),
        |portfolio, transaction| EnergySourcePortfolio::merge(&portfolio, &transaction.portfolio),
    )
}

fn create_emission_event(
    simulation_id: i32,
    timestamp: &Timestamp,
    electrolyzer: &Electrolyzer,
    portfolio: &EnergySourcePortfolio,
) -> EmissionEvent {
    let mut amount_emitted_kg = 0.0;
    amount_emitted_kg += portfolio.natural_gas_mwh * NATURAL_GAS_MWH_TO_CO2;
    amount_emitted_kg += portfolio.coal_mwh * COAL_MWH_TO_CO2;
    amount_emitted_kg += portfolio.petroleum_mwh * PETROLEUM_MWH_TO_CO2;
    amount_emitted_kg += portfolio.biomass_mwh * BIOMASS_MWH_TO_CO2;

    EmissionEvent {
        simulation_id,
        electrolyzer_id: electrolyzer.id,
        emission_timestamp: timestamp.clone(),
        amount_emitted_kg,
    }
}

fn create_hydrogen_production_event(
    simulation_id: i32,
    timestamp: &Timestamp,
    electrolyzer: &Electrolyzer,
    portfolio: &EnergySourcePortfolio,
) -> Result<HydrogenProductionEvent> {
    Ok(HydrogenProductionEvent {
        simulation_id,
        electrolyzer_id: electrolyzer.id,
        production_timestamp: timestamp.clone(),
        kg_hydrogen: f64::min(portfolio.total_electricity_mwh, electrolyzer.capacity_mw)
            * electrolyzer
                .constant_production
                .ok_or_else(|| {
                    Error::create_unimplemented_error("Only constant production supported")
                })?
                .conversion_rate,
    })
}

fn calculate_tax_credit(
    emission: &EmissionEvent,
    hydrogen_production: &HydrogenProductionEvent,
) -> TaxCredit45V {
    let mut tier = TaxCredit45VTier::None;
    let co2_per_h2 = emission.amount_emitted_kg / hydrogen_production.kg_hydrogen;

    if 2.5 <= co2_per_h2 && co2_per_h2 < 4.0 {
        tier = TaxCredit45VTier::Tier3;
    } else if 1.5 <= co2_per_h2 && co2_per_h2 < 2.5 {
        tier = TaxCredit45VTier::Tier2;
    } else if 0.45 <= co2_per_h2 && co2_per_h2 < 1.5 {
        tier = TaxCredit45VTier::Tier1;
    } else if co2_per_h2 < 0.45 {
        tier = TaxCredit45VTier::Max
    }

    let value = tier.value();

    TaxCredit45V {
        tier,
        total_usd: value * hydrogen_production.kg_hydrogen,
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::schema::{
        electrolyzer::{ConstantProduction, Electrolyzer, ProductionType},
        simulation_schema::{
            EmissionEvent, EnergySourcePortfolio, EnergyTransaction, GenerationMetric,
            HydrogenProductionEvent, PowerGrid, PowerPlant, TaxCredit45V, TaxCredit45VTier,
        },
        time::Timestamp,
    };

    use super::{
        calculate_tax_credit, create_emission_event, create_energy_source_portfolio,
        create_hydrogen_production_event, make_optimal_transactions, NATURAL_GAS_MWH_TO_CO2,
    };

    // Note that this is it's own piece of work and should be refactored out
    // Keeping here for now to get working integration test
    #[test]
    fn should_make_optimal_transaction() {
        let simulation_id = 0;
        let timestamp = Timestamp::default();
        let electrolyzer = Electrolyzer::default();
        let mut power_grid = PowerGrid::default();
        let mut power_plant = PowerPlant::default();
        let mut generation_portfolio = EnergySourcePortfolio::default();
        generation_portfolio.total_electricity_mwh = 4.0;
        generation_portfolio.natural_gas_mwh = 4.0;
        let mut transaction_portfolio = EnergySourcePortfolio::default();
        transaction_portfolio.total_electricity_mwh = 2.0;
        transaction_portfolio.natural_gas_mwh = 2.0;
        power_plant.add_generation(GenerationMetric::new(
            0,
            &timestamp,
            1.0,
            generation_portfolio,
        ));
        power_grid.add_power_plant(power_plant);
        let expected_transactions = vec![EnergyTransaction {
            simulation_id: 0,
            electrolyzer_id: 0,
            plant_id: 0,
            timestamp: Timestamp::new(timestamp.seconds, timestamp.nanos),
            price_usd: 2.0,
            portfolio: transaction_portfolio,
        }];

        let transactions =
            make_optimal_transactions(simulation_id, &timestamp, &electrolyzer, &power_grid)
                .unwrap();

        assert_eq!(transactions, expected_transactions);
    }

    // TODO: Refactor this as well like the rest of transaction optimization
    // also should return a result instead of panicing
    #[test]
    #[should_panic]
    fn should_fail_to_make_transaction_missing_generation() {
        let simulation_id = 0;
        let timestamp = Timestamp::default();
        let electrolyzer = Electrolyzer::default();
        let mut power_grid = PowerGrid::default();
        let mut power_plant = PowerPlant::default();
        let mut generation_portfolio = EnergySourcePortfolio::default();
        generation_portfolio.total_electricity_mwh = 4.0;
        generation_portfolio.natural_gas_mwh = 2.0;
        power_plant.add_generation(GenerationMetric::new(
            0,
            &Timestamp::new(timestamp.seconds, timestamp.nanos),
            1.0,
            generation_portfolio,
        ));
        power_grid.add_power_plant(power_plant);
        let future_timestamp = Timestamp::new(timestamp.seconds + 3600, timestamp.nanos);

        make_optimal_transactions(simulation_id, &future_timestamp, &electrolyzer, &power_grid)
            .unwrap();
    }

    #[test]
    fn should_calculate_energy_portfolio() {
        let timestamp = Timestamp::default();
        let mut transaction_portfolio = EnergySourcePortfolio::default();
        transaction_portfolio.total_electricity_mwh = 2.0;
        transaction_portfolio.natural_gas_mwh = 2.0;
        let transactions = vec![
            EnergyTransaction {
                simulation_id: 0,
                electrolyzer_id: 0,
                plant_id: 0,
                timestamp: timestamp.clone(),
                price_usd: 2.0,
                portfolio: transaction_portfolio.clone(),
            },
            EnergyTransaction {
                simulation_id: 0,
                electrolyzer_id: 0,
                plant_id: 0,
                timestamp: timestamp.clone(),
                price_usd: 2.0,
                portfolio: transaction_portfolio.clone(),
            },
        ];
        let mut expected_portfolio = EnergySourcePortfolio::default();
        expected_portfolio.total_electricity_mwh = 4.0;
        expected_portfolio.natural_gas_mwh = 4.0;

        let portfolio = create_energy_source_portfolio(&transactions);

        assert_eq!(portfolio, expected_portfolio);
    }

    #[test]
    fn should_create_emission_event() {
        let simulation_id = 0;
        let electrolyzer = Electrolyzer::default();
        let timestamp = Timestamp::default();
        let mut portfolio = EnergySourcePortfolio::default();
        portfolio.total_electricity_mwh = 4.0;
        portfolio.natural_gas_mwh = 2.0;
        let mut expected_emission_event = EmissionEvent::default();
        expected_emission_event.emission_timestamp =
            Timestamp::new(timestamp.seconds, timestamp.nanos);
        expected_emission_event.amount_emitted_kg = 2.0 * NATURAL_GAS_MWH_TO_CO2;

        let emission_event =
            create_emission_event(simulation_id, &timestamp, &electrolyzer, &portfolio);

        assert_eq!(emission_event, expected_emission_event);
    }

    #[test]
    fn should_create_hydrogen_production_event() {
        let simulation_id = 0;
        let mut electrolyzer = Electrolyzer::default();
        electrolyzer.capacity_mw = 10.0;
        electrolyzer.production_type = ProductionType::Constant;
        electrolyzer.constant_production = Some(ConstantProduction {
            conversion_rate: 2.0,
        });
        let timestamp = Timestamp::default();
        let mut portfolio = EnergySourcePortfolio::default();
        portfolio.total_electricity_mwh = 4.0;
        portfolio.natural_gas_mwh = 2.0;
        let mut expected_hydrogen_production_event = HydrogenProductionEvent::default();
        expected_hydrogen_production_event.production_timestamp = timestamp.clone();
        expected_hydrogen_production_event.kg_hydrogen = 8.0;

        let hydrogen_production_event =
            create_hydrogen_production_event(simulation_id, &timestamp, &electrolyzer, &portfolio)
                .expect("Should create hydrogen production event");

        assert_eq!(
            hydrogen_production_event,
            expected_hydrogen_production_event
        );
    }

    #[test]
    fn should_create_hydrogen_production_event_at_max_capacity() {
        let simulation_id = 0;
        let mut electrolyzer = Electrolyzer::default();
        electrolyzer.capacity_mw = 10.0;
        electrolyzer.production_type = ProductionType::Constant;
        electrolyzer.constant_production = Some(ConstantProduction {
            conversion_rate: 2.0,
        });
        let timestamp = Timestamp::default();
        let mut portfolio = EnergySourcePortfolio::default();
        portfolio.total_electricity_mwh = 14.0;
        portfolio.natural_gas_mwh = 2.0;
        let mut expected_hydrogen_production_event = HydrogenProductionEvent::default();
        expected_hydrogen_production_event.production_timestamp = timestamp.clone();
        expected_hydrogen_production_event.kg_hydrogen = 20.0;

        let hydrogen_production_event =
            create_hydrogen_production_event(simulation_id, &timestamp, &electrolyzer, &portfolio)
                .expect("Should create hydrogen production event");

        assert_eq!(
            hydrogen_production_event,
            expected_hydrogen_production_event
        );
    }

    #[test]
    fn should_calculate_full_tax_credit() {
        let mut emission_event = EmissionEvent::default();
        emission_event.emission_timestamp = Timestamp::default();
        emission_event.amount_emitted_kg = 0.2 * NATURAL_GAS_MWH_TO_CO2;
        let mut hydrogen_production_event = HydrogenProductionEvent::default();
        hydrogen_production_event.production_timestamp = Timestamp::default();
        hydrogen_production_event.kg_hydrogen = 100.0;
        let mut expected_tax_credit = TaxCredit45V::default();
        expected_tax_credit.total_usd = 300.0;
        expected_tax_credit.tier = TaxCredit45VTier::Max;

        let tax_credit = calculate_tax_credit(&emission_event, &hydrogen_production_event);

        assert_eq!(tax_credit, expected_tax_credit);
    }

    #[test]
    fn should_calculate_33_4_percent_tax_credit() {
        let mut emission_event = EmissionEvent::default();
        emission_event.emission_timestamp = Timestamp::default();
        emission_event.amount_emitted_kg = 2.0 * NATURAL_GAS_MWH_TO_CO2;
        let mut hydrogen_production_event = HydrogenProductionEvent::default();
        hydrogen_production_event.production_timestamp = Timestamp::default();
        hydrogen_production_event.kg_hydrogen = 800.0;
        let mut expected_tax_credit = TaxCredit45V::default();
        expected_tax_credit.total_usd = 801.6;
        expected_tax_credit.tier = TaxCredit45VTier::Tier1;

        let tax_credit = calculate_tax_credit(&emission_event, &hydrogen_production_event);

        assert_eq!(tax_credit, expected_tax_credit);
    }

    #[test]
    fn should_calculate_25_percent_tax_credit() {
        let mut emission_event = EmissionEvent::default();
        emission_event.emission_timestamp = Timestamp::default();
        emission_event.amount_emitted_kg = 3.0 * NATURAL_GAS_MWH_TO_CO2;
        let mut hydrogen_production_event = HydrogenProductionEvent::default();
        hydrogen_production_event.production_timestamp = Timestamp::default();
        hydrogen_production_event.kg_hydrogen = 300.0;
        let mut expected_tax_credit = TaxCredit45V::default();
        expected_tax_credit.total_usd = 225.0;
        expected_tax_credit.tier = TaxCredit45VTier::Tier2;

        let tax_credit = calculate_tax_credit(&emission_event, &hydrogen_production_event);

        assert_eq!(tax_credit, expected_tax_credit);
    }

    #[test]
    fn should_calculate_20_percent_tax_credit() {
        let mut emission_event = EmissionEvent::default();
        emission_event.emission_timestamp = Timestamp::default();
        emission_event.amount_emitted_kg = 4.0 * NATURAL_GAS_MWH_TO_CO2;
        let mut hydrogen_production_event = HydrogenProductionEvent::default();
        hydrogen_production_event.production_timestamp = Timestamp::default();
        hydrogen_production_event.kg_hydrogen = 500.0;
        let mut expected_tax_credit = TaxCredit45V::default();
        expected_tax_credit.total_usd = 375.0;
        expected_tax_credit.tier = TaxCredit45VTier::Tier3;

        let tax_credit = calculate_tax_credit(&emission_event, &hydrogen_production_event);

        assert_eq!(tax_credit, expected_tax_credit);
    }
}
