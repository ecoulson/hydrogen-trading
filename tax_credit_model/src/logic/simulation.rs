use crate::schema::{
    electrolyzer::Electrolyzer,
    simulation_schema::{
        EmissionEvent, EnergySourcePortfolio, EnergyTransaction, HydrogenProductionEvent,
        PowerGrid, PowerPlant, SimulationResult, SimulationStatus, TaxCredit45V,
    },
    time::{TimeRange, Timestamp},
    time_series::{TimeSeries, TimeSeriesChart, TimeSeriesEntry},
};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc};

const NATURAL_GAS_MMBTU_TO_CO2: f32 = 53.0703;
const TAX_CREDIT_45V_MAX_VALUE_USD: f32 = 3.0;

struct SimulationState {
    emissions: Vec<EmissionEvent>,
    hydrogen_productions: Vec<HydrogenProductionEvent>,
    transactions: Vec<EnergyTransaction>,
}

impl SimulationState {
    fn new() -> SimulationState {
        SimulationState {
            emissions: vec![],
            hydrogen_productions: vec![],
            transactions: vec![],
        }
    }
}

pub fn simulate(
    power_grid: &PowerGrid,
    electrolyzer: &Electrolyzer,
    time_range: &TimeRange,
) -> SimulationResult {
    let simulation_id = 0;
    let mut current_timestamp = time_range.start.to_utc_date_time().unwrap();
    let end_timestamp = time_range.end.to_utc_date_time().unwrap();
    let increment = Duration::hours(1);
    let mut state = SimulationState::new();

    while current_timestamp < end_timestamp {
        let mut transactions = make_optimal_transactions(
            simulation_id,
            &Timestamp::from(current_timestamp),
            electrolyzer,
            power_grid,
        );
        let portfolio = create_energy_source_portfolio(
            simulation_id,
            &Timestamp::from(current_timestamp),
            &transactions,
        );
        let emission_event = create_emission_event(simulation_id, electrolyzer, &portfolio);
        let hydrogen_production_event =
            create_hydrogen_production_event(simulation_id, electrolyzer, &portfolio);

        state.transactions.append(&mut transactions);
        state.emissions.push(emission_event);
        state.hydrogen_productions.push(hydrogen_production_event);

        current_timestamp += increment;
    }

    let tax_credit = calculate_tax_credit(&state.emissions, &state.hydrogen_productions);

    let emission_time_series = TimeSeries {
        label: String::from("Emissions"),
        data_points: state
            .emissions
            .iter()
            .map(|emission| TimeSeriesEntry {
                date: emission
                    .emission_timestamp
                    .to_utc_date_time()
                    .unwrap()
                    .to_rfc3339(),
                value: emission.amount_emitted_kg,
            })
            .collect(),
    };
    let hydrogen_production_time_series = TimeSeries {
        label: String::from("Hydrogen Produced"),
        data_points: state
            .hydrogen_productions
            .iter()
            .map(|production| TimeSeriesEntry {
                date: production
                    .production_timestamp
                    .to_utc_date_time()
                    .unwrap()
                    .to_rfc3339(),
                value: production.kg_hydrogen,
            })
            .collect(),
    };

    SimulationResult {
        status: SimulationStatus::Complete,
        tax_credit,
        emissions: TimeSeriesChart {
            id: String::from("emissions"),
            title: String::from("Emission time series"),
            time_series: emission_time_series,
        },
        hydrogen_productions: TimeSeriesChart {
            id: String::from("hydrogen-produced"),
            title: String::from("Hydrogen Production over time"),
            time_series: hydrogen_production_time_series,
        },
    }
}

fn make_optimal_transactions(
    simulation_id: i32,
    timestamp: &Timestamp,
    electrolyzer: &Electrolyzer,
    power_grid: &PowerGrid,
) -> Vec<EnergyTransaction> {
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
    amount_mwh: f32,
    timestamp: &Timestamp,
) -> EnergyTransaction {
    let purchase_datetime = Utc
        .timestamp_opt(timestamp.seconds, timestamp.nanos)
        .unwrap();
    let generation = power_plant
        .generation
        .iter()
        .find(|generation| {
            let generation_datetime = Utc
                .timestamp_opt(
                    generation.time_generated.seconds,
                    generation.time_generated.nanos,
                )
                .unwrap();

            purchase_datetime.year() == generation_datetime.year()
                && purchase_datetime.month() == generation_datetime.month()
                && purchase_datetime.day() == generation_datetime.day()
                && purchase_datetime.hour() == generation_datetime.hour()
        })
        .expect("Should find a valid generation");

    EnergyTransaction {
        simulation_id,
        electrolyzer_id: electrolyzer.id,
        plant_id: generation.plant_id,
        timestamp: Timestamp::new(timestamp.seconds, timestamp.nanos),
        amount_mwh,
        fuel_consumed_mmbtu: amount_mwh * (1.0 / power_plant.heat_rate),
        price_usd: generation.sale_price_usd_per_mwh * amount_mwh,
        energy_source: power_plant.energy_source.clone(),
    }
}

fn create_energy_source_portfolio(
    simulation_id: i32,
    timestamp: &Timestamp,
    transactions: &Vec<EnergyTransaction>,
) -> EnergySourcePortfolio {
    let mut portfolio = EnergySourcePortfolio::new(
        simulation_id,
        Timestamp::new(timestamp.seconds, timestamp.nanos),
    );
    for transaction in transactions {
        portfolio.total_electricity_mwh += transaction.amount_mwh;
        portfolio.natural_gas_mmbtu += transaction.fuel_consumed_mmbtu;
    }

    portfolio
}

fn create_emission_event(
    simulation_id: i32,
    electrolyzer: &Electrolyzer,
    portfolio: &EnergySourcePortfolio,
) -> EmissionEvent {
    let mut amount_emitted_kg = 0.0;
    amount_emitted_kg += portfolio.natural_gas_mmbtu * NATURAL_GAS_MMBTU_TO_CO2;

    EmissionEvent {
        simulation_id,
        electrolyzer_id: electrolyzer.id,
        emission_timestamp: Timestamp::new(portfolio.timestamp.seconds, portfolio.timestamp.nanos),
        amount_emitted_kg,
    }
}

fn create_hydrogen_production_event(
    simulation_id: i32,
    electrolyzer: &Electrolyzer,
    portfolio: &EnergySourcePortfolio,
) -> HydrogenProductionEvent {
    HydrogenProductionEvent {
        simulation_id,
        electrolyzer_id: electrolyzer.id,
        production_timestamp: Timestamp::new(
            portfolio.timestamp.seconds,
            portfolio.timestamp.nanos,
        ),
        kg_hydrogen: f32::min(portfolio.total_electricity_mwh, electrolyzer.capacity_mw)
            * electrolyzer
                .constant_production
                .expect("Only constant production supported")
                .conversion_rate,
    }
}

fn calculate_tax_credit(
    emissions: &Vec<EmissionEvent>,
    hydrogen_productions: &Vec<HydrogenProductionEvent>,
) -> TaxCredit45V {
    let mut amount_usd_per_kg = 0.0;
    let total_co2_emitted = emissions
        .iter()
        .fold(0.0, |sum, emission| sum + emission.amount_emitted_kg);
    let total_h2_produced = hydrogen_productions
        .iter()
        .fold(0.0, |sum, hydrogen_production| {
            sum + hydrogen_production.kg_hydrogen
        });
    let co2_per_h2 = total_co2_emitted / total_h2_produced;

    if 2.5 <= co2_per_h2 && co2_per_h2 < 4.0 {
        amount_usd_per_kg = TAX_CREDIT_45V_MAX_VALUE_USD * 0.2
    } else if 1.5 <= co2_per_h2 && co2_per_h2 < 2.5 {
        amount_usd_per_kg = TAX_CREDIT_45V_MAX_VALUE_USD * 0.25
    } else if 0.45 <= co2_per_h2 && co2_per_h2 < 1.5 {
        amount_usd_per_kg = TAX_CREDIT_45V_MAX_VALUE_USD * 0.334
    } else if co2_per_h2 < 0.45 {
        amount_usd_per_kg = TAX_CREDIT_45V_MAX_VALUE_USD
    }

    TaxCredit45V {
        amount_usd_per_kg,
        total_usd: amount_usd_per_kg * total_h2_produced,
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::schema::{
        electrolyzer::{ConstantProduction, Electrolyzer, ProductionType},
        simulation_schema::{
            EmissionEvent, EnergySource, EnergySourcePortfolio, EnergyTransaction,
            GenerationMetric, HydrogenProductionEvent, PowerGrid, PowerPlant, TaxCredit45V,
        },
        time::Timestamp,
    };

    use super::{
        calculate_tax_credit, create_emission_event, create_energy_source_portfolio,
        create_hydrogen_production_event, make_optimal_transactions, NATURAL_GAS_MMBTU_TO_CO2,
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
        power_plant.energy_source = EnergySource::NaturalGas;
        power_plant.heat_rate = 2.0; // mmbtu / mwh
        power_plant.add_generation(GenerationMetric::new(
            0,
            &Timestamp::new(timestamp.seconds, timestamp.nanos),
            4.0,
            1.0,
            2.0,
        ));
        power_grid.add_power_plant(power_plant);
        let expected_transactions = vec![EnergyTransaction {
            simulation_id: 0,
            electrolyzer_id: 0,
            plant_id: 0,
            timestamp: Timestamp::new(timestamp.seconds, timestamp.nanos),
            amount_mwh: 2.0,
            fuel_consumed_mmbtu: 1.0,
            price_usd: 2.0,
            energy_source: EnergySource::NaturalGas,
        }];

        let transactions =
            make_optimal_transactions(simulation_id, &timestamp, &electrolyzer, &power_grid);

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
        power_plant.energy_source = EnergySource::NaturalGas;
        power_plant.heat_rate = 2.0; // mmbtu / mwh
        power_plant.add_generation(GenerationMetric::new(
            0,
            &Timestamp::new(timestamp.seconds, timestamp.nanos),
            4.0,
            1.0,
            2.0,
        ));
        power_grid.add_power_plant(power_plant);
        let future_timestamp = Timestamp::new(timestamp.seconds + 3600, timestamp.nanos);

        make_optimal_transactions(simulation_id, &future_timestamp, &electrolyzer, &power_grid);
    }

    #[test]
    fn should_calculate_energy_portfolio() {
        let simulation_id = 0;
        let timestamp = Timestamp::default();
        let transactions = vec![
            EnergyTransaction {
                simulation_id: 0,
                electrolyzer_id: 0,
                plant_id: 0,
                timestamp: Timestamp::new(timestamp.seconds, timestamp.nanos),
                amount_mwh: 2.0,
                fuel_consumed_mmbtu: 1.0,
                price_usd: 2.0,
                energy_source: EnergySource::NaturalGas,
            },
            EnergyTransaction {
                simulation_id: 0,
                electrolyzer_id: 0,
                plant_id: 0,
                timestamp: Timestamp::new(timestamp.seconds, timestamp.nanos),
                amount_mwh: 2.0,
                fuel_consumed_mmbtu: 1.0,
                price_usd: 2.0,
                energy_source: EnergySource::NaturalGas,
            },
        ];
        let mut expected_portfolio = EnergySourcePortfolio::new(
            simulation_id,
            Timestamp::new(timestamp.seconds, timestamp.nanos),
        );
        expected_portfolio.total_electricity_mwh = 4.0;
        expected_portfolio.natural_gas_mmbtu = 2.0;

        let portfolio = create_energy_source_portfolio(simulation_id, &timestamp, &transactions);

        assert_eq!(portfolio, expected_portfolio);
    }

    #[test]
    fn should_create_emission_event() {
        let simulation_id = 0;
        let electrolyzer = Electrolyzer::default();
        let timestamp = Timestamp::default();
        let mut portfolio = EnergySourcePortfolio::new(
            simulation_id,
            Timestamp::new(timestamp.seconds, timestamp.nanos),
        );
        portfolio.total_electricity_mwh = 4.0;
        portfolio.natural_gas_mmbtu = 2.0;
        let mut expected_emission_event = EmissionEvent::default();
        expected_emission_event.emission_timestamp =
            Timestamp::new(timestamp.seconds, timestamp.nanos);
        expected_emission_event.amount_emitted_kg = 2.0 * NATURAL_GAS_MMBTU_TO_CO2;

        let emission_event = create_emission_event(simulation_id, &electrolyzer, &portfolio);

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
        let mut portfolio = EnergySourcePortfolio::new(
            simulation_id,
            Timestamp::new(timestamp.seconds, timestamp.nanos),
        );
        portfolio.total_electricity_mwh = 4.0;
        portfolio.natural_gas_mmbtu = 2.0;
        let mut expected_hydrogen_production_event = HydrogenProductionEvent::default();
        expected_hydrogen_production_event.production_timestamp =
            Timestamp::new(timestamp.seconds, timestamp.nanos);
        expected_hydrogen_production_event.kg_hydrogen = 8.0;

        let hydrogen_production_event =
            create_hydrogen_production_event(simulation_id, &electrolyzer, &portfolio);

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
        let mut portfolio = EnergySourcePortfolio::new(
            simulation_id,
            Timestamp::new(timestamp.seconds, timestamp.nanos),
        );
        portfolio.total_electricity_mwh = 14.0;
        portfolio.natural_gas_mmbtu = 2.0;
        let mut expected_hydrogen_production_event = HydrogenProductionEvent::default();
        expected_hydrogen_production_event.production_timestamp =
            Timestamp::new(timestamp.seconds, timestamp.nanos);
        expected_hydrogen_production_event.kg_hydrogen = 20.0;

        let hydrogen_production_event =
            create_hydrogen_production_event(simulation_id, &electrolyzer, &portfolio);

        assert_eq!(
            hydrogen_production_event,
            expected_hydrogen_production_event
        );
    }

    #[test]
    fn should_calculate_full_tax_credit() {
        let mut emission_event = EmissionEvent::default();
        emission_event.emission_timestamp = Timestamp::default();
        emission_event.amount_emitted_kg = 0.2 * NATURAL_GAS_MMBTU_TO_CO2;
        let emissions = vec![emission_event];
        let mut hydrogen_production_event = HydrogenProductionEvent::default();
        hydrogen_production_event.production_timestamp = Timestamp::default();
        hydrogen_production_event.kg_hydrogen = 100.0;
        let hydrogen_productions = vec![hydrogen_production_event];
        let mut expected_tax_credit = TaxCredit45V::default();
        expected_tax_credit.total_usd = 300.0;
        expected_tax_credit.amount_usd_per_kg = 3.0;

        let tax_credit = calculate_tax_credit(&emissions, &hydrogen_productions);

        assert_eq!(tax_credit, expected_tax_credit);
    }

    #[test]
    fn should_calculate_33_4_percent_tax_credit() {
        let mut emission_event = EmissionEvent::default();
        emission_event.emission_timestamp = Timestamp::default();
        emission_event.amount_emitted_kg = 2.0 * NATURAL_GAS_MMBTU_TO_CO2;
        let emissions = vec![emission_event];
        let mut hydrogen_production_event = HydrogenProductionEvent::default();
        hydrogen_production_event.production_timestamp = Timestamp::default();
        hydrogen_production_event.kg_hydrogen = 100.0;
        let hydrogen_productions = vec![hydrogen_production_event];
        let mut expected_tax_credit = TaxCredit45V::default();
        expected_tax_credit.total_usd = 100.2;
        expected_tax_credit.amount_usd_per_kg = 1.002;

        let tax_credit = calculate_tax_credit(&emissions, &hydrogen_productions);

        assert_eq!(tax_credit, expected_tax_credit);
    }

    #[test]
    fn should_calculate_25_percent_tax_credit() {
        let mut emission_event = EmissionEvent::default();
        emission_event.emission_timestamp = Timestamp::default();
        emission_event.amount_emitted_kg = 3.0 * NATURAL_GAS_MMBTU_TO_CO2;
        let emissions = vec![emission_event];
        let mut hydrogen_production_event = HydrogenProductionEvent::default();
        hydrogen_production_event.production_timestamp = Timestamp::default();
        hydrogen_production_event.kg_hydrogen = 100.0;
        let hydrogen_productions = vec![hydrogen_production_event];
        let mut expected_tax_credit = TaxCredit45V::default();
        expected_tax_credit.total_usd = 75.0;
        expected_tax_credit.amount_usd_per_kg = 0.75;

        let tax_credit = calculate_tax_credit(&emissions, &hydrogen_productions);

        assert_eq!(tax_credit, expected_tax_credit);
    }

    #[test]
    fn should_calculate_20_percent_tax_credit() {
        let mut emission_event = EmissionEvent::default();
        emission_event.emission_timestamp = Timestamp::default();
        emission_event.amount_emitted_kg = 4.0 * NATURAL_GAS_MMBTU_TO_CO2;
        let emissions = vec![emission_event];
        let mut hydrogen_production_event = HydrogenProductionEvent::default();
        hydrogen_production_event.production_timestamp = Timestamp::default();
        hydrogen_production_event.kg_hydrogen = 100.0;
        let hydrogen_productions = vec![hydrogen_production_event];
        let mut expected_tax_credit = TaxCredit45V::default();
        expected_tax_credit.total_usd = 75.0;
        expected_tax_credit.amount_usd_per_kg = 0.75;

        let tax_credit = calculate_tax_credit(&emissions, &hydrogen_productions);

        assert_eq!(tax_credit, expected_tax_credit);
    }
}
