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

#[derive(Debug, Default)]
pub struct FormFieldBuilder {
    form_field: FormField,
}

impl FormFieldBuilder {
    pub fn new() -> Self {
        Self {
            form_field: FormField::default(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.form_field.name = name;

        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.form_field.label = label;

        self
    }

    pub fn variant(mut self, variant: FormFieldVariant) -> Self {
        self.form_field.variant = variant;

        self
    }

    pub fn unit(mut self, unit: Badge) -> Self {
        self.form_field.unit = Some(unit);

        self
    }

    pub fn select(mut self, select: Select) -> Self {
        self.form_field.select = select;

        self
    }

    pub fn input(mut self, input: Input) -> Self {
        self.form_field.input = input;

        self
    }

    pub fn build(self) -> FormField {
        self.form_field
    }
}
