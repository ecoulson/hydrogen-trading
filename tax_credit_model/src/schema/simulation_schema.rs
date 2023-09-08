use askama::Template;
use rocket::FromForm;
use serde::{Deserialize, Serialize};

use super::{
    time::{TimeRange, Timestamp},
    time_series::TimeSeriesChart,
};

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct ExecuteSimulationRequest {
    pub electrolyzer_id: usize,
    pub simulation_time_range: TimeRange,
}

impl ExecuteSimulationRequest {
    pub fn new(
        electrolyzer_id: usize,
        simulation_time_range: TimeRange,
    ) -> ExecuteSimulationRequest {
        ExecuteSimulationRequest {
            electrolyzer_id,
            simulation_time_range,
        }
    }
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/execute_simulation.html")]
pub struct ExecuteSimulationResponse {
    pub simulation_result: SimulationResult,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub enum SimulationStatus {
    #[default]
    Complete,
}

impl std::fmt::Display for SimulationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Complete => write!(f, "Complete"),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct SimulationResult {
    pub status: SimulationStatus,
    pub tax_credit: TaxCredit45V,
    pub emissions: TimeSeriesChart,
    pub hydrogen_productions: TimeSeriesChart,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct EmissionEvent {
    pub simulation_id: i32,
    pub electrolyzer_id: usize,
    pub emission_timestamp: Timestamp,
    pub amount_emitted_kg: f32,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct HydrogenProductionEvent {
    pub simulation_id: i32,
    pub electrolyzer_id: usize,
    pub production_timestamp: Timestamp,
    pub kg_hydrogen: f32,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct EnergySourcePortfolio {
    pub simulation_id: i32,
    pub timestamp: Timestamp,
    pub total_electricity_mwh: f32,
    pub petroleum_mmbtu: f32,
    pub hydrocarbons_mmbtu: f32,
    pub natural_gas_mmbtu: f32,
    pub coal_mmbtu: f32,
    pub nuclear_mmbtu: f32,
    pub solar_mmbtu: f32,
    pub geothermal_mmbtu: f32,
    pub wind_mmbtu: f32,
    pub biomass_mmbtu: f32,
    pub hydropower_mmbtu: f32,
}

impl EnergySourcePortfolio {
    pub fn new(simulation_id: i32, timestamp: Timestamp) -> EnergySourcePortfolio {
        EnergySourcePortfolio {
            simulation_id,
            timestamp,
            total_electricity_mwh: 0.0,
            petroleum_mmbtu: 0.0,
            hydrocarbons_mmbtu: 0.0,
            natural_gas_mmbtu: 0.0,
            coal_mmbtu: 0.0,
            nuclear_mmbtu: 0.0,
            solar_mmbtu: 0.0,
            geothermal_mmbtu: 0.0,
            wind_mmbtu: 0.0,
            biomass_mmbtu: 0.0,
            hydropower_mmbtu: 0.0,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct PowerGrid {
    pub power_plants: Vec<PowerPlant>,
}

impl PowerGrid {
    pub fn add_power_plant(&mut self, power_plant: PowerPlant) {
        self.power_plants.push(power_plant);
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub enum PolicyType {
    #[default]
    PPAAgreement,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub struct Policy {
    pub policy_type: PolicyType,
    pub ppa_agreement: PPAAgreement,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub struct PPAAgreement {
    pub plant_id: i32,
    pub electrolyzer_id: usize,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub enum EnergySource {
    #[default]
    Petroleum,
    Hydrocarbons,
    NaturalGas,
    Coal,
    Nuclear,
    Solar,
    Geothermal,
    Wind,
    Biomass,
    Hydropower,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct GenerationMetric {
    pub plant_id: i32,
    pub time_generated: Timestamp,
    pub amount_mwh: f32,
    pub sale_price_usd_per_mwh: f32,
    pub amount_mmbtu: f32,
}

impl GenerationMetric {
    pub fn new(
        plant_id: i32,
        time_generated: &Timestamp,
        amount_mwh: f32,
        sale_price_usd_per_mwh: f32,
        amount_mmbtu: f32,
    ) -> GenerationMetric {
        GenerationMetric {
            plant_id,
            time_generated: Timestamp::new(time_generated.seconds, time_generated.nanos),
            amount_mmbtu,
            sale_price_usd_per_mwh,
            amount_mwh,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct PowerPlant {
    pub plant_id: i32,
    pub energy_source: EnergySource,
    pub heat_rate: f32,
    pub generation: Vec<GenerationMetric>,
}

impl PowerPlant {
    pub fn add_generation(&mut self, generation: GenerationMetric) {
        self.generation.push(generation);
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct TaxCredit45V {
    pub amount_usd_per_kg: f32,
    pub total_usd: f32,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct EnergyTransaction {
    pub simulation_id: i32,
    pub electrolyzer_id: usize,
    pub plant_id: i32,
    pub timestamp: Timestamp,
    pub amount_mwh: f32,
    pub fuel_consumed_mmbtu: f32,
    pub price_usd: f32,
    pub energy_source: EnergySource,
}
