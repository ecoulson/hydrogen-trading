use askama::Template;
use rocket::{FromForm, FromFormField};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
pub struct ConstantProduction {
    pub conversion_rate: f32,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
pub struct Electrolyzer {
    pub id: usize,
    pub replacement_threshold: f32,
    pub degredation_rate: f32,
    pub capacity_mw: f32,
    pub production_type: ProductionType,
    pub constant_production: Option<ConstantProduction>,
    pub capex: f32,
    pub opex: f32,
    pub replacement_cost: f32,
}

impl Electrolyzer {
    pub fn constant_production(
        id: usize,
        replacement_threshold: f32,
        replacement_cost: f32,
        degredation_rate: f32,
        capacity_mw: f32,
        production_rate: f32,
        capex: f32,
        opex: f32,
    ) -> Electrolyzer {
        Electrolyzer {
            id,
            replacement_threshold,
            replacement_cost,
            degredation_rate,
            capacity_mw,
            production_type: ProductionType::Constant,
            constant_production: Some(ConstantProduction {
                conversion_rate: production_rate,
            }),
            capex,
            opex,
        }
    }
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
    pub conversion_rate_constant: Option<f32>,
}

#[derive(FromForm, Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
pub struct CreateElectrolyzerRequest {
    pub replacement_threshold: f32,
    pub degredation_rate: f32,
    pub capacity_mw: f32,
    pub production_method: CreateProductionRequest,
    pub capex: f32,
    pub opex: f32,
    pub replacement_cost: f32,
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
#[template(path = "components/create_electrolyzer.html")]
pub struct CreateElectrolyzerResponse {
    pub electrolyzer: Electrolyzer,
}
