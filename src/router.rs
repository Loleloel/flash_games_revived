use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::window;

use crate::pages::{home::HomePage, games::GamesPage, about::AboutPage, not_found::NotFoundPage};

// We'll use a conditional compilation approach to handle GitHub Pages
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

// This function will provide the correct base path for the BrowserRouter
pub fn get_base_url() -> String {
    // Try to get the current location from the window
    if let Some(window) = window() {
        if let Ok(location) = window.location().pathname() {
            // If we're on GitHub Pages, the path will contain the repository name
            if location.contains("/flash_games_revived") {
                return "/flash_games_revived".to_string();
            }
        }
    }
    
    // Default for local development
    "/".to_string()
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Games => html! { <GamesPage /> },
        Route::About => html! { <AboutPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}