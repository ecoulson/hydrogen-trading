use askama::Template;
use rocket::{FromForm, FromFormField};
use serde::{Deserialize, Serialize};

use crate::{
    client::events::ClientEvent,
    components::{
        badge::{Badge, BadgeBuilder, BadgeVariant},
        button::{Button, ButtonBuilder},
        event::{EventListener, EventListenerBuilder},
    },
};

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

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/electrolyzer_details.html")]
pub struct ElectrolyzerDetails {
    electrolyzer: Electrolyzer,
    state: ElectrolyzerDetailsState,
    list_simulations_listener: EventListener,
    select_simulation_listener: EventListener,
    select_electrolyzer_button: Button,
    conversion_rate_badge: Badge,
    opex_badge: Badge,
    capex_badge: Badge,
    capacity_badge: Badge,
    degradation_rate_badge: Badge,
    replacement_threshold_badge: Badge,
    replacement_cost_badge: Badge,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Clone)]
pub enum ElectrolyzerDetailsState {
    Selected(Badge),
    #[default]
    Default,
}

pub struct ElectrolyzerDetailsBuilder {
    electrolyzer_details: ElectrolyzerDetails,
}

impl ElectrolyzerDetailsBuilder {
    pub fn new() -> Self {
        Self {
            electrolyzer_details: ElectrolyzerDetails {
                list_simulations_listener: EventListenerBuilder::new()
                    .event(ClientEvent::ListSimulations)
                    .endpoint("/list_electrolyzers")
                    .target("#sidebar")
                    .build(),
                select_simulation_listener: EventListenerBuilder::new()
                    .event(ClientEvent::SelectSimulation)
                    .endpoint("/get_selected_electrolyzer")
                    .target("#sidebar")
                    .build(),
                electrolyzer: Electrolyzer::default(),
                state: ElectrolyzerDetailsState::Default,
                select_electrolyzer_button: ButtonBuilder::new()
                    .endpoint("/select_electrolyzer")
                    .target("#sidebar")
                    .text("Use")
                    .build(),
                opex_badge: BadgeBuilder::new()
                    .variant(BadgeVariant::Secondary)
                    .text("$ / Hour")
                    .build(),
                capex_badge: BadgeBuilder::new()
                    .variant(BadgeVariant::Secondary)
                    .text("$")
                    .build(),
                capacity_badge: BadgeBuilder::new()
                    .variant(BadgeVariant::Secondary)
                    .text("MW")
                    .build(),
                degradation_rate_badge: BadgeBuilder::new()
                    .variant(BadgeVariant::Secondary)
                    .text("% / Year")
                    .build(),
                replacement_threshold_badge: BadgeBuilder::new()
                    .variant(BadgeVariant::Secondary)
                    .text("%")
                    .build(),
                replacement_cost_badge: BadgeBuilder::new()
                    .variant(BadgeVariant::Secondary)
                    .text("$ / Replacement")
                    .build(),
                conversion_rate_badge: BadgeBuilder::new()
                    .variant(BadgeVariant::Secondary)
                    .text("kg / MW")
                    .build(),
            },
        }
    }

    pub fn electrolyzer(mut self, electrolyzer: Electrolyzer) -> Self {
        self.electrolyzer_details.electrolyzer = electrolyzer;

        self
    }

    pub fn selected(mut self) -> Self {
        self.electrolyzer_details.state =
            ElectrolyzerDetailsState::Selected(BadgeBuilder::new().text("Selected").build());
        self.electrolyzer_details
            .select_electrolyzer_button
            .disable();

        self
    }

    pub fn state(mut self, state: ElectrolyzerDetailsState) -> Self {
        self.electrolyzer_details.state = state;

        self
    }

    pub fn select_electrolyzer_button(mut self, button: Button) -> Self {
        self.electrolyzer_details.select_electrolyzer_button = button;

        self
    }

    pub fn build(self) -> ElectrolyzerDetails {
        self.electrolyzer_details
    }
}
