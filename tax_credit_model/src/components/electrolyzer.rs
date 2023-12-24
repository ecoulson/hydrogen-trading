use askama::Template;

use crate::{
    client::{events::ClientEvent, htmx::HtmxSwap},
    schema::{
        electrolyzer::{Electrolyzer, ElectrolyzerDetailsState, ElectrolyzerId},
        endpoints::Endpoint,
    },
};

use super::{
    badge::Badge,
    button::{Button, ButtonVariant},
    event::EventListener,
    icon::{Icon, IconColor, IconKind, IconSize},
    input::Input,
};

#[derive(Template, Default, Debug)]
#[template(path = "components/create_electrolyzer.html")]
pub struct CreateElectrolyzerForm {
    endpoint: Endpoint,
    create_electrolyzer_button: Button,
    conversion_rate_badge: Badge,
    opex_badge: Badge,
    capex_badge: Badge,
    capacity_badge: Badge,
    degradation_rate_badge: Badge,
    replacement_threshold_badge: Badge,
    replacement_cost_badge: Badge,
    left_arrow_icon: Icon,
}

impl CreateElectrolyzerForm {
    pub fn render() -> Self {
        Self {
            endpoint: Endpoint::ListElectrolyzers,
            opex_badge: Badge::render("$ / Hour"),
            capex_badge: Badge::render("$"),
            capacity_badge: Badge::render("MW"),
            degradation_rate_badge: Badge::render("% / Year"),
            replacement_threshold_badge: Badge::render("%"),
            replacement_cost_badge: Badge::render("$ / replacement"),
            conversion_rate_badge: Badge::render("kg / MW"),
            create_electrolyzer_button: Button::render(
                "Create Electrolyzer",
                Endpoint::CreateElectrolyzer,
                "#sidebar",
            ),
            left_arrow_icon: Icon::render_filled(
                IconKind::LeftArrow,
                IconSize::Small,
                IconColor::Black,
            ),
        }
    }
}

#[derive(Template, Default, Debug)]
#[template(path = "components/electrolyzer_details.html")]
pub struct ElectrolyzerDetails {
    pub electrolyzer: Electrolyzer,
    pub state: ElectrolyzerDetailsState,
    pub list_simulations_listener: EventListener,
    pub select_simulation_listener: EventListener,
    pub state_icon: Icon,
    pub select_electrolyzer_button: Button,
    pub conversion_rate_badge: Badge,
    pub opex_badge: Badge,
    pub capex_badge: Badge,
    pub capacity_badge: Badge,
    pub degradation_rate_badge: Badge,
    pub replacement_threshold_badge: Badge,
    pub replacement_cost_badge: Badge,
    pub left_arrow_icon: Icon,
    pub electrolyzer_id_input: Input,
    pub endpoint: Endpoint,
}

impl ElectrolyzerDetails {
    pub fn render_unselected(electrolyzer: Electrolyzer) -> Self {
        Self::render(electrolyzer, ElectrolyzerDetailsState::Default)
    }

    pub fn render_selected(electrolyzer: Electrolyzer) -> Self {
        let mut details = Self::render(
            electrolyzer,
            ElectrolyzerDetailsState::Selected(Badge::render_info("Selected")),
        );
        details.select_electrolyzer_button.variant = ButtonVariant::Disabled;

        details
    }

    pub fn render_default(electrolyzer: Electrolyzer) -> Self {
        let mut details = Self::render(electrolyzer, ElectrolyzerDetailsState::Default);
        details.select_electrolyzer_button.variant = ButtonVariant::Disabled;

        details
    }

    pub fn render(electrolyzer: Electrolyzer, state: ElectrolyzerDetailsState) -> Self {
        let id = &electrolyzer.id.to_string();

        ElectrolyzerDetails {
            endpoint: Endpoint::ListElectrolyzers,
            list_simulations_listener: EventListener::render(
                ClientEvent::ListSimulations,
                Endpoint::ListElectrolyzers,
                "#sidebar",
                HtmxSwap::InnerHtml,
            ),
            select_simulation_listener: EventListener::render(
                ClientEvent::SelectSimulation,
                Endpoint::GetSelectedElectrolyzer,
                "#sidebar",
                HtmxSwap::InnerHtml,
            ),
            electrolyzer,
            state,
            state_icon: Icon::render_filled(IconKind::Texas, IconSize::Small, IconColor::Black),
            select_electrolyzer_button: Button::render(
                "Use",
                Endpoint::SelectElectrolyzer,
                "#sidebar",
            ),
            opex_badge: Badge::render("$ / hour"),
            capex_badge: Badge::render("$"),
            capacity_badge: Badge::render("MW"),
            degradation_rate_badge: Badge::render("% / year"),
            replacement_threshold_badge: Badge::render("%"),
            replacement_cost_badge: Badge::render("$ / replacement"),
            conversion_rate_badge: Badge::render("kg / MW"),
            left_arrow_icon: Icon::render_filled(
                IconKind::LeftArrow,
                IconSize::Small,
                IconColor::Black,
            ),
            electrolyzer_id_input: Input::render_hidden(id, "electrolyzer_id"),
        }
    }
}

#[derive(Template, Default, Debug)]
#[template(path = "components/electrolyzer_selector.html")]
pub struct ElectrolyzerSelector {
    pub endpoint: Endpoint,
    pub selected_id: ElectrolyzerId,
    pub electrolyzers: Vec<Electrolyzer>,
    pub select_electrolyzer_listener: EventListener,
}

impl ElectrolyzerSelector {
    pub fn render(selected_id: ElectrolyzerId, electrolyzers: Vec<Electrolyzer>) -> Self {
        Self {
            endpoint: Endpoint::SelectElectrolyzer,
            selected_id,
            electrolyzers,
            select_electrolyzer_listener: EventListener::render(
                ClientEvent::SelectElectrolyzer,
                Endpoint::ElectrolyzerSelector,
                "#electrolyzer-selector",
                HtmxSwap::OuterHtml,
            ),
        }
    }
}

#[derive(Template, Default, Debug)]
#[template(path = "components/list_electrolyzers.html")]
pub struct ElectrolyzerList {
    pub search_results: ElectrolyzerSearchResults,
    pub list_simulation_listener: EventListener,
    pub select_simulation_listener: EventListener,
    pub create_electrolyzer_button: Button,
    pub search_bar: Input,
}

impl ElectrolyzerList {
    pub fn render(electrolyzers: Vec<Electrolyzer>) -> Self {
        ElectrolyzerList {
            search_results: ElectrolyzerSearchResults::render(electrolyzers),
            select_simulation_listener: EventListener::render(
                ClientEvent::SelectSimulation,
                Endpoint::GetSelectedElectrolyzer,
                "#sidebar",
                HtmxSwap::InnerHtml,
            ),
            list_simulation_listener: EventListener::render(
                ClientEvent::ListSimulations,
                Endpoint::ListElectrolyzers,
                "#sidebar",
                HtmxSwap::InnerHtml,
            ),
            create_electrolyzer_button: Button::render(
                "Create Electrolyzer",
                Endpoint::GetCreateElectrolyzerForm,
                "#sidebar",
            ),
            search_bar: Input::render_search(
                "query",
                Endpoint::SearchElectrolyzers,
                "Search for electrolyzers",
                "#electrolyzer-search-results",
            ),
        }
    }

    pub fn render_selected(selected_id: ElectrolyzerId, electrolyzers: Vec<Electrolyzer>) -> Self {
        ElectrolyzerList {
            search_results: ElectrolyzerSearchResults::render_selected(selected_id, electrolyzers),
            select_simulation_listener: EventListener::render(
                ClientEvent::SelectSimulation,
                Endpoint::GetSelectedElectrolyzer,
                "#sidebar",
                HtmxSwap::InnerHtml,
            ),
            list_simulation_listener: EventListener::render(
                ClientEvent::ListSimulations,
                Endpoint::ListElectrolyzers,
                "#sidebar",
                HtmxSwap::InnerHtml,
            ),
            create_electrolyzer_button: Button::render(
                "Create Electrolyzer",
                Endpoint::GetCreateElectrolyzerForm,
                "#sidebar",
            ),
            search_bar: Input::render_search(
                "query",
                Endpoint::SearchElectrolyzers,
                "Search for electrolyzers",
                "#electrolyzer-search-results",
            ),
        }
    }
}

#[derive(Template, Default, Debug)]
#[template(path = "components/electrolyzer_search_results.html")]
pub struct ElectrolyzerSearchResults {
    pub results: Vec<ElectrolyzerSearchResultItem>,
}

impl ElectrolyzerSearchResults {
    pub fn render(electrolyzers: Vec<Electrolyzer>) -> Self {
        let results = electrolyzers
            .into_iter()
            .enumerate()
            .map(|(i, electrolyzer)| {
                ElectrolyzerSearchResultItem::render(
                    i + 1,
                    electrolyzer,
                    ElectrolyzerSearchResultItemState::Selected,
                )
            })
            .collect();

        Self { results }
    }

    pub fn render_selected(selected_id: ElectrolyzerId, electrolyzers: Vec<Electrolyzer>) -> Self {
        let results = electrolyzers
            .into_iter()
            .enumerate()
            .map(|(i, electrolyzer)| {
                let state = ElectrolyzerSearchResultItemState::get(&electrolyzer.id, &selected_id);
                ElectrolyzerSearchResultItem::render(i + 1, electrolyzer, state)
            })
            .collect();

        Self { results }
    }
}

#[derive(Default, Debug)]
pub enum ElectrolyzerSearchResultItemState {
    #[default]
    Unselected,
    Selected,
}

impl ElectrolyzerSearchResultItemState {
    pub fn get(id: &ElectrolyzerId, selected_id: &ElectrolyzerId) -> Self {
        if id == selected_id {
            ElectrolyzerSearchResultItemState::Selected
        } else {
            ElectrolyzerSearchResultItemState::Unselected
        }
    }
}

#[derive(Template, Default, Debug)]
#[template(path = "components/electrolyzer_search_result_item.html")]
pub struct ElectrolyzerSearchResultItem {
    pub index: usize,
    pub id_input: Input,
    pub state: ElectrolyzerSearchResultItemState,
    pub electrolyzer: Electrolyzer,
    pub select_electrolyzer_button: Button,
    pub state_icon: Icon,
    pub endpoint: Endpoint,
}

impl ElectrolyzerSearchResultItem {
    pub fn render(
        index: usize,
        electrolyzer: Electrolyzer,
        state: ElectrolyzerSearchResultItemState,
    ) -> Self {
        Self {
            index,
            id_input: Input::render_hidden(&electrolyzer.id.to_string(), "electrolyzer_id"),
            electrolyzer,
            state,
            select_electrolyzer_button: Button::render_outline(
                "Select",
                Endpoint::SelectElectrolyzer,
                "#sidebar",
            ),
            state_icon: Icon::render_filled(IconKind::Texas, IconSize::Small, IconColor::Black),
            endpoint: Endpoint::GetElectrolyzer,
        }
    }
}
