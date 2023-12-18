use rocket::{FromForm, FromFormField};
use serde::{Deserialize, Serialize};

use crate::components::badge::Badge;

pub type ElectrolyzerId = usize;

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
pub struct ConstantProduction {
    pub conversion_rate: f64,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct Electrolyzer {
    pub id: ElectrolyzerId,
    pub replacement_threshold: f64,
    pub degradation_rate: f64,
    pub capacity_mw: f64,
    pub production_type: ProductionType,
    pub production: ConstantProduction,
    pub capex: f64,
    pub opex: f64,
    pub replacement_cost: f64,
    pub name: String,
    pub state: String,
    pub city: String,
}

impl Electrolyzer {
    pub fn constant_production(
        id: ElectrolyzerId,
        name: &str,
        replacement_threshold: f64,
        replacement_cost: f64,
        degradation_rate: f64,
        capacity_mw: f64,
        production_rate: f64,
        capex: f64,
        opex: f64,
    ) -> Electrolyzer {
        Electrolyzer {
            id,
            name: String::from(name),
            replacement_threshold,
            replacement_cost,
            degradation_rate,
            capacity_mw,
            production_type: ProductionType::Constant,
            production: ConstantProduction {
                conversion_rate: production_rate,
            },
            capex,
            opex,
            city: String::from("Huston"),
            state: String::from("TX"),
        }
    }
}

#[derive(FromForm, Debug, Default)]
pub struct SelectElectrolyzerHandlerRequest {
    pub electrolyzer_id: ElectrolyzerId,
}


#[derive(FromForm)]
pub struct GetElectrolyzerRequest {
    pub electrolyzer_id: ElectrolyzerId,
}


#[derive(FromFormField, Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
pub enum ProductionType {
    #[default]
    Constant,
    Variable,
}

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
pub struct CreateProductionRequest {
    pub production_type: ProductionType,
    pub conversion_rate_constant: Option<f64>,
}

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub struct CreateElectrolyzerRequest {
    pub replacement_threshold: f64,
    pub degradation_rate: f64,
    pub capacity_mw: f64,
    pub production_method: CreateProductionRequest,
    pub capex: f64,
    pub opex: f64,
    pub replacement_cost: f64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub enum ElectrolyzerDetailsState {
    #[default]
    Default,
    Unselected,
    Selected(Badge),
}

#[derive(FromForm)]
pub struct SearchElectrolyzersRequest {
    pub query: String,
}

