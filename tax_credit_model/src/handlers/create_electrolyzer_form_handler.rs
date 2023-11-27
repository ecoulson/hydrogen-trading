use askama::Template;
use rocket::post;
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        badge::{Badge, BadgeBuilder, BadgeVariant},
        button::{Button, ButtonBuilder},
        component::{Component, ComponentResponse},
    },
    schema::errors::BannerError,
};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/create_electrolyzer.html")]
pub struct CreateElectrolyzerForm {
    create_electrolyzer_button: Button,
    conversion_rate_badge: Badge,
    opex_badge: Badge,
    capex_badge: Badge,
    capacity_badge: Badge,
    degradation_rate_badge: Badge,
    replacement_threshold_badge: Badge,
    replacement_cost_badge: Badge,
}

pub struct CreateElectrolyzerFormBuilder {
    create_electrolyzer_from: CreateElectrolyzerForm,
}

impl CreateElectrolyzerFormBuilder {
    pub fn new() -> Self {
        Self {
            create_electrolyzer_from: CreateElectrolyzerForm {
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
                create_electrolyzer_button: ButtonBuilder::new()
                    .text("Create Electrolyzer")
                    .endpoint("/create_electrolyzer")
                    .target("#sidebar")
                    .build(),
            },
        }
    }

    pub fn build(self) -> CreateElectrolyzerForm {
        self.create_electrolyzer_from
    }
}

#[post("/create_electrolyzer_form")]
pub fn create_electrolyzer_form_handler() -> ComponentResponse<CreateElectrolyzerForm, BannerError>
{
    Component::basic(CreateElectrolyzerFormBuilder::new().build())
}
