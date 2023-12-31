use std::str::FromStr;

use rocket::FromForm;
use serde::{Deserialize, Serialize};

use crate::components::{histogram::HistogramResponse, time_series::TimeSeriesChartResponse};

use super::{
    electrolyzer::ElectrolyzerId,
    errors::{Error, Result},
    time::{DateTimeRange, Timestamp},
};

pub type SimulationId = usize;
pub type PowerPlantId = usize;

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct ExecuteSimulationRequest {
    pub electrolyzer_id: ElectrolyzerId,
    pub simulation_time_range: DateTimeRange,
}

impl ExecuteSimulationRequest {
    pub fn new(electrolyzer_id: ElectrolyzerId, simulation_time_range: DateTimeRange) -> Self {
        Self {
            electrolyzer_id,
            simulation_time_range,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
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

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct TaxCreditSummary {
    pub credit_hours_full: f64,
    pub credit_hours_33: f64,
    pub credit_hours_25: f64,
    pub credit_hours_20: f64,
    pub credit_hours_none: f64,
}

#[derive(Default, Debug)]
pub struct SimulationResult {
    pub hourly_histogram: HistogramResponse,
    pub tax_credit_summary: TaxCreditSummary,
    pub emissions: TimeSeriesChartResponse,
    pub hydrogen_productions: TimeSeriesChartResponse,
    pub energy_costs: TimeSeriesChartResponse,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct EmissionEvent {
    pub simulation_id: SimulationId,
    pub electrolyzer_id: ElectrolyzerId,
    pub emission_timestamp: Timestamp,
    pub amount_emitted_kg: f64,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct HydrogenProductionEvent {
    pub simulation_id: SimulationId,
    pub electrolyzer_id: ElectrolyzerId,
    pub production_timestamp: Timestamp,
    pub kg_hydrogen: f64,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
pub struct EnergySourcePortfolio {
    pub total_electricity_mwh: f64,
    pub petroleum_mwh: f64,
    pub hydrocarbons_mwh: f64,
    pub natural_gas_mwh: f64,
    pub coal_mwh: f64,
    pub nuclear_mwh: f64,
    pub solar_mwh: f64,
    pub geothermal_mwh: f64,
    pub wind_mwh: f64,
    pub biomass_mwh: f64,
    pub hydropower_mwh: f64,
    pub unknown_mwh: f64,
    pub wholesale_storage_load: f64,
}

impl EnergySourcePortfolio {
    pub fn add_energy(&mut self, source: &EnergySource, amount_mwh: f64) {
        self.total_electricity_mwh += amount_mwh;

        match source {
            EnergySource::Coal => self.coal_mwh += amount_mwh,
            EnergySource::NaturalGas => self.natural_gas_mwh += amount_mwh,
            EnergySource::Solar => self.solar_mwh += amount_mwh,
            EnergySource::Petroleum => self.petroleum_mwh += amount_mwh,
            EnergySource::Hydropower => self.hydropower_mwh += amount_mwh,
            EnergySource::Hydrocarbons => self.hydrocarbons_mwh += amount_mwh,
            EnergySource::Nuclear => self.nuclear_mwh += amount_mwh,
            EnergySource::Geothermal => self.geothermal_mwh += amount_mwh,
            EnergySource::Wind => self.wind_mwh += amount_mwh,
            EnergySource::Biomass => self.biomass_mwh += amount_mwh,
            EnergySource::WholesaleStorageLoad => self.wholesale_storage_load += amount_mwh,
            EnergySource::Unknown => self.unknown_mwh += amount_mwh,
        }
    }

    pub fn merge(
        portfolio_a: &EnergySourcePortfolio,
        portfolio_b: &EnergySourcePortfolio,
    ) -> EnergySourcePortfolio {
        EnergySourcePortfolio {
            total_electricity_mwh: portfolio_a.total_electricity_mwh
                + portfolio_b.total_electricity_mwh,
            coal_mwh: portfolio_a.coal_mwh + portfolio_b.coal_mwh,
            natural_gas_mwh: portfolio_a.natural_gas_mwh + portfolio_b.natural_gas_mwh,
            solar_mwh: portfolio_a.solar_mwh + portfolio_b.solar_mwh,
            petroleum_mwh: portfolio_a.petroleum_mwh + portfolio_b.petroleum_mwh,
            hydropower_mwh: portfolio_a.hydropower_mwh + portfolio_b.hydropower_mwh,
            hydrocarbons_mwh: portfolio_a.hydrocarbons_mwh + portfolio_b.hydrocarbons_mwh,
            nuclear_mwh: portfolio_a.nuclear_mwh + portfolio_b.nuclear_mwh,
            geothermal_mwh: portfolio_a.geothermal_mwh + portfolio_b.geothermal_mwh,
            wind_mwh: portfolio_a.wind_mwh + portfolio_b.wind_mwh,
            biomass_mwh: portfolio_a.biomass_mwh + portfolio_b.biomass_mwh,
            wholesale_storage_load: portfolio_a.wholesale_storage_load
                + portfolio_b.wholesale_storage_load,
            unknown_mwh: portfolio_a.unknown_mwh + portfolio_b.unknown_mwh,
        }
    }

    pub fn scale_to_amount(
        portfolio: &EnergySourcePortfolio,
        amount_mwh: f64,
    ) -> Result<EnergySourcePortfolio> {
        if amount_mwh > portfolio.total_electricity_mwh {
            return Err(Error::invalid_argument("Total electricity exceeded"));
        }

        let scale_factor = amount_mwh / portfolio.total_electricity_mwh;

        Ok(EnergySourcePortfolio {
            total_electricity_mwh: amount_mwh,
            coal_mwh: portfolio.coal_mwh * scale_factor,
            natural_gas_mwh: portfolio.natural_gas_mwh * scale_factor,
            solar_mwh: portfolio.solar_mwh * scale_factor,
            petroleum_mwh: portfolio.petroleum_mwh * scale_factor,
            hydropower_mwh: portfolio.hydropower_mwh * scale_factor,
            hydrocarbons_mwh: portfolio.hydrocarbons_mwh * scale_factor,
            nuclear_mwh: portfolio.nuclear_mwh * scale_factor,
            geothermal_mwh: portfolio.geothermal_mwh * scale_factor,
            wind_mwh: portfolio.wind_mwh * scale_factor,
            biomass_mwh: portfolio.biomass_mwh * scale_factor,
            wholesale_storage_load: portfolio.wholesale_storage_load * scale_factor,
            unknown_mwh: portfolio.unknown_mwh * scale_factor,
        })
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct PowerGrid {
    pub power_plants: Vec<PowerPlant>,
}

impl PowerGrid {
    pub fn add_power_plant(&mut self, power_plant: PowerPlant) {
        self.power_plants.push(power_plant);
    }
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
    WholesaleStorageLoad,
    Unknown,
}

impl FromStr for EnergySource {
    type Err = Error;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "Coal" => Ok(EnergySource::Coal),
            "Biomass" => Ok(EnergySource::Biomass),
            "Gas" => Ok(EnergySource::NaturalGas),
            "Gas-CC" => Ok(EnergySource::NaturalGas),
            "Hydro" => Ok(EnergySource::Hydropower),
            "Nuclear" => Ok(EnergySource::Nuclear),
            "Solar" => Ok(EnergySource::Solar),
            "Wind" => Ok(EnergySource::Wind),
            "WSL" => Ok(EnergySource::WholesaleStorageLoad),
            "Other" => Ok(EnergySource::Unknown),
            _ => Err(Error::invalid_argument(value)),
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct GenerationMetric {
    pub plant_id: PowerPlantId,
    pub time_generated: Timestamp,
    pub sale_price_usd_per_mwh: f64,
    pub portfolio: EnergySourcePortfolio,
}

impl GenerationMetric {
    pub fn new(
        plant_id: PowerPlantId,
        time_generated: &Timestamp,
        sale_price_usd_per_mwh: f64,
        portfolio: EnergySourcePortfolio,
    ) -> Self {
        Self {
            plant_id,
            time_generated: Timestamp::new(time_generated.seconds, time_generated.nanos),
            sale_price_usd_per_mwh,
            portfolio,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct PowerPlant {
    pub plant_id: PowerPlantId,
    pub generations: Vec<GenerationMetric>,
}

impl PowerPlant {
    pub fn add_generation(&mut self, generation: GenerationMetric) {
        self.generations.push(generation);
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum TaxCredit45VTier {
    Max,
    Tier1,
    Tier2,
    Tier3,
    #[default]
    None,
}

impl TaxCredit45VTier {
    pub fn value(&self) -> f64 {
        match self {
            Self::Max => 3.0,
            Self::Tier1 => 3.0 * 0.334,
            Self::Tier2 => 3.0 * 0.25,
            Self::Tier3 => 3.0 * 0.2,
            Self::None => 0.0,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct TaxCredit45V {
    pub tier: TaxCredit45VTier,
    pub total_usd: f64,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct EnergyTransaction {
    pub simulation_id: SimulationId,
    pub electrolyzer_id: ElectrolyzerId,
    pub plant_id: PowerPlantId,
    pub timestamp: Timestamp,
    pub price_usd: f64,
    pub portfolio: EnergySourcePortfolio,
}
