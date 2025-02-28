use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::HomePage, games::GamesPage, about::AboutPage, not_found::NotFoundPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/games")]
    Games,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Games => html! { <GamesPage /> },
        Route::About => html! { <AboutPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}