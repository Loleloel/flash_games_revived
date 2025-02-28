use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="not-found-container arcade-section">
            <div class="retro-panel error-panel">
                <h1 class="glitch-text">{"SYSTEM ERROR 404"}</h1>
                <div class="retro-line"></div>
                <div class="terminal-section">
                    <div class="terminal-header">
                        <span class="blink-text">{"> "}</span>
                        <span class="command-text">{"locate requested_page"}</span>
                    </div>
                    <p class="terminal-output error-message">
                        {"ERROR: Target page not found in database"}<br/>
                        {"SYSTEM MESSAGE: The requested resource has been deleted or never existed"}
                    </p>
                    <div class="error-details">
                        <p class="error-code">{"[ERROR_CODE: 0x404]"}</p>
                        <p class="error-time">{format!("TIMESTAMP: {}", "2184.13.32.59.99")}</p>
                    </div>
                </div>
                <div class="error-animation">
                    <div class="glitch-block"></div>
                    <div class="glitch-block"></div>
                    <div class="glitch-block"></div>
                </div>
                <div class="not-found-actions">
                    <Link<Route> classes="neon-link" to={Route::Home}>
                        {">> RETURN TO MAIN TERMINAL"}
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}