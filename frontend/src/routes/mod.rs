use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;
pub mod wheel;

use about::About;
use home::Home;
use wheel::Wheel;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/wheel")]
    Wheel,
    #[at("/")]
    Home,
}

/// Switch app routes
#[allow(clippy::let_unit_value)]
pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::Wheel => html! { <Wheel /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
