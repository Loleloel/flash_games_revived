use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Route, switch, get_base_url};

#[function_component(App)]
pub fn app() -> Html {
    // Get the base URL that adapts to GitHub Pages or local development
    let base_url = get_base_url();
    
    html! {
        <BrowserRouter basename={base_url}>
            <div class="app-container">
                <header class="app-header">
                    <div class="header-left">
                        <h1>{"Flash Games Revived"}</h1>
                        <nav class="cyberpunk-nav">
                            <Link<Route> to={Route::Home}>
                                <span class="nav-text">{"Home"}</span>
                                <span class="nav-glow"></span>
                            </Link<Route>>
                            <Link<Route> to={Route::Games}>
                                <span class="nav-text">{"Games"}</span>
                                <span class="nav-glow"></span>
                            </Link<Route>>
                            <Link<Route> to={Route::About}>
                                <span class="nav-text">{"About"}</span>
                                <span class="nav-glow"></span>
                            </Link<Route>>
                        </nav>
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