use std::collections::HashMap;

use crate::{
    components::{histogram::HistogramResponse, time_series::TimeSeriesChartResponse},
    persistance::simulation::SimulationClient,
    schema::{
        electrolyzer::{Electrolyzer, ElectrolyzerId},
        endpoints::Endpoint,
        errors::{Error, Result},
        histogram::{Histogram, HistogramDataset, Labels},
        simulation::{
            EmissionEvent, EnergySourcePortfolio, EnergyTransaction, HydrogenProductionEvent,
            PowerGrid, PowerPlant, SimulationId, SimulationResult, TaxCredit45V, TaxCredit45VTier,
            TaxCreditSummary,
        },
        time::{DateTimeRange, Timestamp},
        time_series::{ChartColor, TimeSeries, TimeSeriesChart, TimeSeriesEntry},
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
    pub id: SimulationId,
    pub electrolyzer_id: ElectrolyzerId,
    pub emissions: Vec<EmissionEvent>,
    pub hydrogen_productions: Vec<HydrogenProductionEvent>,
    pub transactions: Vec<EnergyTransaction>,
    pub tax_credit: Vec<TaxCredit45V>,
    pub tax_credit_summary: TaxCreditSummary,
}

pub fn simulate(
    simulation_id: SimulationId,
    power_grid: &PowerGrid,
    electrolyzer: &Electrolyzer,
    time_range: &DateTimeRange,
    simulation_client: &Box<dyn SimulationClient>,
) -> Result<SimulationResult> {
    let time_range = time_range.parse("%Y-%m-%dT%H:%M")?;
    let mut current_timestamp = time_range.start.to_utc_date_time()?;
    let mut end_timestamp = time_range.end.to_utc_date_time()?;
    let increment = Duration::minutes(15);
    let mut state = simulation_client.get_simulation_state(&simulation_id)?;
    state.electrolyzer_id = electrolyzer.id;

    if current_timestamp.timestamp() > end_timestamp.timestamp() {
        return Err(Error::invalid_argument(
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

    simulation_client.update(&state)?;
    let mut energy_costs_time_series = TimeSeries {
        color: ChartColor::Blue,
        label: String::from("Energy Cost"),
        data_points: state
            .transactions
            .iter()
            .fold(HashMap::new(), |mut aggregation, transaction| {
                if let Some(current_price) = aggregation.get_mut(&transaction.timestamp) {
                    *current_price += transaction.price_usd;
                } else {
                    aggregation.insert(transaction.timestamp, transaction.price_usd);
                }

                aggregation
            })
            .iter()
            .map(|(key, value)| TimeSeriesEntry::render(*value, key, ChartColor::Blue))
            .collect::<Result<Vec<TimeSeriesEntry>>>()?,
    };
    energy_costs_time_series
        .data_points
        .sort_by(|a, b| a.date.cmp(&b.date));

    Ok(SimulationResult {
        tax_credit_summary: state.tax_credit_summary.clone(),
        emissions: produce_emissions_graph(&state)?,
        hydrogen_productions: TimeSeriesChartResponse::render(
            TimeSeriesChart::render(
                "Hydrogen Production Over Time",
                Labels::render("Simulation Date", "kg (H2O)"),
                vec![TimeSeries::render(
                    "Energy Cost",
                    ChartColor::Blue,
                    state.hydrogen_productions,
                    |production| {
                        TimeSeriesEntry::render(
                            production.kg_hydrogen,
                            &production.production_timestamp,
                            ChartColor::Blue,
                        )
                    },
                )?],
            ),
            Endpoint::FetchHydrogenProduction,
            HashMap::from([("simulation_id", simulation_id.to_string())]),
        ),
        energy_costs: TimeSeriesChartResponse::render(
            TimeSeriesChart::render(
                "Energy Costs Over Time",
                Labels::render("Simulation Date", "USD ($)"),
                vec![energy_costs_time_series],
            ),
            Endpoint::FetchEnergyCosts,
            HashMap::from([("simulation_id", simulation_id.to_string())]),
        ),
        hourly_histogram: HistogramResponse::render(
            Endpoint::FetchHourlyHistogram,
            HashMap::from([("simulation_id", simulation_id.to_string())]),
            Histogram::render(
                "Hourly Tax Credits",
                Labels::render("Tax Credit Level", "Hours"),
                vec!["0%", "20%", "25%", "33%", "100%"],
                vec![HistogramDataset::render(
                    "Credit Breakdown",
                    vec![
                        state.tax_credit_summary.credit_hours_none,
                        state.tax_credit_summary.credit_hours_20,
                        state.tax_credit_summary.credit_hours_25,
                        state.tax_credit_summary.credit_hours_33,
                        state.tax_credit_summary.credit_hours_full,
                    ],
                )],
            ),
        ),
    })
}

fn produce_emissions_graph(state: &SimulationState) -> Result<TimeSeriesChartResponse> {
    Ok(TimeSeriesChartResponse::render(
        TimeSeriesChart::render(
            "Emissions Over Time",
            Labels::render("Simulation Date", "kg (CO2)"),
            vec![TimeSeries::render(
                "CO2 Emissions",
                ChartColor::Blue,
                state.emissions.iter().zip(&state.tax_credit).collect(),
                |(emission, tax_credit)| {
                    TimeSeriesEntry::render(
                        emission.amount_emitted_kg,
                        &emission.emission_timestamp,
                        match tax_credit.tier {
                            TaxCredit45VTier::Max => ChartColor::Green,
                            TaxCredit45VTier::Tier1 => ChartColor::Chartreuse,
                            TaxCredit45VTier::Tier2 => ChartColor::Yellow,
                            TaxCredit45VTier::Tier3 => ChartColor::Orange,
                            TaxCredit45VTier::None => ChartColor::Red,
                        },
                    )
                },
            )?],
        ),
        Endpoint::FetchEmissions,
        HashMap::from([("simulation_id", state.id.to_string())]),
    ))
}

fn make_optimal_transactions(
    simulation_id: SimulationId,
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
    simulation_id: SimulationId,
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
        .ok_or_else(|| Error::not_found(&format!("Generation not found for timestep")))?;

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
    simulation_id: SimulationId,
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
    simulation_id: SimulationId,
    timestamp: &Timestamp,
    electrolyzer: &Electrolyzer,
    portfolio: &EnergySourcePortfolio,
) -> Result<HydrogenProductionEvent> {
    Ok(HydrogenProductionEvent {
        simulation_id,
        electrolyzer_id: electrolyzer.id,
        production_timestamp: timestamp.clone(),
        kg_hydrogen: f64::min(portfolio.total_electricity_mwh, electrolyzer.capacity_mw)
            * electrolyzer.production.conversion_rate,
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
        simulation::{
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
        electrolyzer.production = ConstantProduction {
            conversion_rate: 2.0,
        };
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
        electrolyzer.production = ConstantProduction {
            conversion_rate: 2.0,
        };
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
        emission_event.amount_emitted_kg = 8.0 * NATURAL_GAS_MWH_TO_CO2;
        let mut hydrogen_production_event = HydrogenProductionEvent::default();
        hydrogen_production_event.production_timestamp = Timestamp::default();
        hydrogen_production_event.kg_hydrogen = 500.0;
        let mut expected_tax_credit = TaxCredit45V::default();
        expected_tax_credit.total_usd =
            hydrogen_production_event.kg_hydrogen * TaxCredit45VTier::Tier3.value();
        expected_tax_credit.tier = TaxCredit45VTier::Tier3;

        let tax_credit = calculate_tax_credit(&emission_event, &hydrogen_production_event);

        assert_eq!(tax_credit.tier, expected_tax_credit.tier);
    }
}
