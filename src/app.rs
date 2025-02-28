use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use web_sys::window;

use crate::router::{Route, switch};
use crate::components::theme_toggle::ThemeToggle;

const THEME_KEY: &str = "flash_games_theme";

#[function_component(App)]
pub fn app() -> Html {
    let theme_state = use_state(|| {
        LocalStorage::get(THEME_KEY).unwrap_or(false)
    });

    {
        let is_dark = *theme_state;
        use_effect_with_deps(move |_| {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.document_element() {
                        if is_dark {
                            element.set_attribute("data-theme", "dark").ok();
                        } else {
                            element.set_attribute("data-theme", "light").ok();
                        }
                    }
                }
            }
            || ()
        }, *theme_state);
    }

    let on_toggle = {
        let theme_state = theme_state.clone();
        Callback::from(move |_| {
            let new_theme = !*theme_state;
            LocalStorage::set(THEME_KEY, new_theme).unwrap_or_default();
            theme_state.set(new_theme);
        })
    };

    html! {
        <BrowserRouter>
            <div class="app-container">
                <header class="app-header">
                    <div class="header-left">
                        <h1>{"Flash Games Revived"}</h1>
                        <nav>
                            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                            <Link<Route> to={Route::Games}>{"Games"}</Link<Route>>
                            <Link<Route> to={Route::About}>{"About"}</Link<Route>>
                        </nav>
                    </div>
                    <div class="header-right">
                        <ThemeToggle on_toggle={on_toggle} is_dark={*theme_state} />
                    </div>
                </header>
                <main>
                    <Switch<Route> render={switch} />
                </main>
                <footer>
                    <p>{"© Flash Games Revived - A modern WASM gaming platform"}</p>
                </footer>
            </div>
        </BrowserRouter>
    }
}