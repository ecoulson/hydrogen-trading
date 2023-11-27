use std::fmt::Display;

use askama::Template;

#[derive(Debug, Default, PartialEq)]
pub enum IconKind {
    #[default]
    Close,
    LeftArrow,
    Texas,
}

#[derive(Debug, Default, PartialEq)]
pub enum IconStroke {
    #[default]
    None,
    Stroke(IconColor),
}

impl Display for IconStroke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            IconStroke::None => write!(f, ""),
            IconStroke::Stroke(color) => write!(f, "stroke-{color}"),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum IconFill {
    #[default]
    None,
    Fill(IconColor),
}

impl Display for IconFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            IconFill::None => write!(f, ""),
            IconFill::Fill(color) => write!(f, "fill-{color}"),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum IconColor {
    #[default]
    None,
    Black,
}

impl Display for IconColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            IconColor::None => write!(f, ""),
            IconColor::Black => write!(f, "black"),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum IconSize {
    #[default]
    Small,
}

impl Display for IconSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            IconSize::Small => write!(f, "w-6 h-6"),
        }
    }
}

#[derive(Template, Debug, Default, PartialEq)]
#[template(path = "components/icon.html")]
pub struct Icon {
    fill: IconFill,
    stroke: IconStroke,
    size: IconSize,
    kind: IconKind,
}

#[derive(Debug)]
pub struct IconBuilder {
    icon: Icon,
}

impl IconBuilder {
    pub fn new() -> Self {
        Self {
            icon: Icon::default(),
        }
    }

    pub fn fill(mut self, fill: IconColor) -> Self {
        self.icon.fill = IconFill::Fill(fill);

        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.icon.size = size;

        self
    }

    pub fn stroke(mut self, stroke: IconColor) -> Self {
        self.icon.stroke = IconStroke::Stroke(stroke);

        self
    }

    pub fn kind(mut self, kind: IconKind) -> Self {
        self.icon.kind = kind;

        self
    }

    pub fn build(self) -> Icon {
        self.icon
    }
}
