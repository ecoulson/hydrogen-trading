use askama::Template;
use rocket::post;
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        button::{Button, ButtonBuilder},
        component::{Component, ComponentResponse},
    },
    schema::errors::BannerError,
};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/create_electrolyzer.html")]
pub struct CreateElectrolyzerForm {
    create_electrolyzer_button: Button,
}

pub struct CreateElectrolyzerFormBuilder {
    create_electrolyzer_from: CreateElectrolyzerForm,
}

impl CreateElectrolyzerFormBuilder {
    pub fn new() -> Self {
        Self {
            create_electrolyzer_from: CreateElectrolyzerForm {
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
