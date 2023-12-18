use rocket::post;

use crate::{
    components::{
        component::{Component, ComponentResponse},
        electrolyzer::CreateElectrolyzerForm,
    },
    schema::errors::BannerError,
};

#[post("/create_electrolyzer_form")]
pub fn create_electrolyzer_form_handler() -> ComponentResponse<CreateElectrolyzerForm, BannerError>
{
    Component::basic(CreateElectrolyzerForm::render())
}
