use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum HtmxSwap {
    #[default]
    InnerHtml,
    OuterHtml,
}

impl Display for HtmxSwap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::OuterHtml => write!(f, "outerHTML"),
            Self::InnerHtml => write!(f, "innerHTML")
        }
    }
}
