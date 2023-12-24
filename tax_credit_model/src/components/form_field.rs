use askama::Template;

use super::{badge::Badge, input::Input, select::Select};

#[derive(Debug, Default)]
pub enum FormFieldVariant {
    #[default]
    Input,
    Select,
}

#[derive(Template, Debug, Default)]
#[template(path = "components/form_field.html")]
pub struct FormField {
    variant: FormFieldVariant,
    unit: Option<Badge>,
    label: String,
    name: String,
    input: Input,
    select: Select,
}

