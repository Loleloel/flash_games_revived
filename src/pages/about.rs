use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="about-container arcade-section">
            <div class="retro-panel main-panel">
                <h1 class="glitch-text">{"SYS.INFO"}</h1>
                <div class="retro-line"></div>
                
                <div class="terminal-section">
                    <div class="terminal-header">
                        <span class="blink-text">{"> "}</span>
                        <span class="command-text">{"cat mission.txt"}</span>
                    </div>
                    <p class="terminal-output">
                        {"FLASH GAMES REVIVED: A project dedicated to preserving the legacy of browser gaming through WebAssembly technology. Our mission is to ensure that the creative spirit of the Flash era lives on in a modern, secure environment."}
                    </p>
                </div>

                <div class="info-grid">
                    <div class="info-panel">
                        <h2 class="section-title">{"CORE.SYSTEMS"}</h2>
                        <ul class="tech-list">
                            <li>{">> Rust/WASM Architecture"}</li>
                            <li>{">> Modern Browser Support"}</li>
                            <li>{">> High Performance Engine"}</li>
                            <li>{">> Secure Sandbox Environment"}</li>
                        </ul>
                    </div>

                    <div class="info-panel">
                        <h2 class="section-title">{"DATA.STATS"}</h2>
                        <ul class="stats-list">
                            <li>{"UPTIME: "}<span class="highlight-text">{"99.9%"}</span></li>
                            <li>{"SECURITY: "}<span class="highlight-text">{"MAXIMUM"}</span></li>
                            <li>{"NOSTALGIA: "}<span class="highlight-text">{"INFINITE"}</span></li>
                        </ul>
                    </div>
                </div>

                <div class="terminal-section">
                    <div class="terminal-header">
                        <span class="blink-text">{"> "}</span>
                        <span class="command-text">{"cat contribute.txt"}</span>
                    </div>
                    <div class="terminal-output">
                        <p>{"JOIN THE REVOLUTION"}</p>
                        <p>{"We're looking for developers, artists, and gaming enthusiasts to help preserve gaming history."}</p>
                        <a href="mailto:join@flashgamesrevived.example" class="neon-link">{">> INITIALIZE CONTACT SEQUENCE"}</a>
                    </div>
                </div>

                <div class="system-status">
                    <p class="status-text">
                        {"SYSTEM STATUS: "}
                        <span class="blink-text online-status">{"OPERATIONAL"}</span>
                    </p>
                    <p class="version-text">{"v1.0.0-alpha"}</p>
                </div>
            </div>
        </div>
    }
}