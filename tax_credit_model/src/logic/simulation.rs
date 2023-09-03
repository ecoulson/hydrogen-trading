use crate::schema::{simulation_schema::{
    EmissionEvent, EnergySourcePortfolio, EnergyTransaction, HydrogenProductionEvent, PowerGrid,
    PowerPlant, SimulationResult, SimulationStatus, TaxCredit45V, TimeRange, Timestamp,
}, electrolyzer::Electrolyzer};
use chrono::{Datelike, Duration, TimeZone, Utc};

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
    let mut current_timestamp = Utc
        .timestamp_opt(time_range.start.seconds, time_range.start.nanos)
        .unwrap();
    let end_timestamp = Utc
        .timestamp_opt(time_range.end.seconds, time_range.end.nanos)
        .unwrap();
    let increment = Duration::hours(1);
    let mut state = SimulationState::new();

    while current_timestamp < end_timestamp {
        let mut transactions = make_optimal_transactions(
            simulation_id,
            &Timestamp {
                seconds: current_timestamp.timestamp(),
                nanos: current_timestamp.timestamp_subsec_nanos(),
            },
            electrolyzer,
            power_grid,
        );
        let portfolio = create_energy_source_portfolio(
            simulation_id,
            Timestamp {
                seconds: current_timestamp.timestamp(),
                nanos: current_timestamp.timestamp_subsec_nanos(),
            },
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

    let tax_credit = calculate_tax_credit(&state);

    SimulationResult {
        status: SimulationStatus::Complete,
        tax_credit,
        hydrogen_productions: state.hydrogen_productions,
        emissions: state.emissions,
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
        .map(|power_plant| purchase(simulation_id, electrolyzer, power_plant, 1.0, timestamp))
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
        })
        .expect("Should find a valid generation");

    EnergyTransaction {
        simulation_id,
        electrolyzer_id: electrolyzer.id,
        plant_id: generation.plant_id,
        timestamp: Timestamp {
            seconds: timestamp.seconds,
            nanos: timestamp.nanos,
        },
        amount_mwh: generation.amount_mwh,
        fuel_consumed_mmbtu: amount_mwh * power_plant.heat_rate,
        price_usd: generation.sale_price_usd_per_mwh * amount_mwh,
        energy_source: power_plant.energy_source.clone(),
    }
}

fn create_energy_source_portfolio(
    simulation_id: i32,
    timestamp: Timestamp,
    transactions: &Vec<EnergyTransaction>,
) -> EnergySourcePortfolio {
    let mut portfolio = EnergySourcePortfolio::new(
        simulation_id,
        Timestamp {
            seconds: timestamp.seconds,
            nanos: timestamp.nanos,
        },
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
        emission_timestamp: Timestamp {
            seconds: portfolio.timestamp.seconds,
            nanos: portfolio.timestamp.nanos,
        },
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
        production_timestamp: Timestamp {
            seconds: portfolio.timestamp.seconds,
            nanos: portfolio.timestamp.nanos,
        },
        kg_hydrogen: f32::min(portfolio.total_electricity_mwh, electrolyzer.capacity_mw)
            * electrolyzer.production_method.conversion_rate,
    }
}

fn calculate_tax_credit(state: &SimulationState) -> TaxCredit45V {
    let mut amount_usd_per_kg = 0.0;
    let total_co2_emitted = state
        .emissions
        .iter()
        .fold(0.0, |sum, emission| sum + emission.amount_emitted_kg);
    let total_h2_produced = state
        .hydrogen_productions
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
