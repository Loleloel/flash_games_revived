use yew::prelude::*;
use crate::components::game_card::{GameCard, GameCardProps};

#[function_component(GamesPage)]
pub fn games_page() -> Html {
    let games = vec![
        GameCardProps {
            title: "CYBER STRIKE".to_string(),
            description: Some("Battle through cyberspace in this retro-futuristic shooter".to_string()),
            thumbnail: None,
            game_id: None,
        },
        GameCardProps {
            title: "PIXELBOT ARENA".to_string(),
            description: Some("Fast-paced robot combat with retro graphics".to_string()),
            thumbnail: None,
            game_id: None,
        },
        GameCardProps {
            title: "NEON MAZE".to_string(),
            description: Some("Navigate through procedurally generated neon labyrinths".to_string()),
            thumbnail: None,
            game_id: None,
        },
        GameCardProps {
            title: "SYNTHWAVE RACER".to_string(),
            description: Some("High-speed racing in a retro-future world".to_string()),
            thumbnail: None,
            game_id: None,
        },
    ];

    html! {
        <div class="games-container arcade-section">
            <div class="games-header">
                <h1 class="retro-title">{"GAME TERMINAL"}</h1>
                <div class="retro-line"></div>
                <p class="terminal-text">{"ACCESSING GAME DATABASE..."}</p>
            </div>
            
            <div class="games-filter retro-panel">
                <div class="filter-group">
                    <span class="filter-label">{"SEARCH:"}</span>
                    <input type="text" placeholder="Enter game title..." class="retro-input" />
                </div>
                <div class="filter-group">
                    <span class="filter-label">{"GENRE:"}</span>
                    <select class="retro-select">
                        <option>{"ALL GAMES"}</option>
                        <option>{"ACTION"}</option>
                        <option>{"ADVENTURE"}</option>
                        <option>{"PUZZLE"}</option>
                        <option>{"STRATEGY"}</option>
                    </select>
                </div>
            </div>
            
            <div class="game-grid">
                {
                    if games.is_empty() {
                        html! {
                            <div class="empty-state retro-panel">
                                <p class="blink-text">{"NO GAMES FOUND IN DATABASE"}</p>
                                <p>{"INITIALIZATION IN PROGRESS..."}</p>
                            </div>
                        }
                    } else {
                        games.iter().map(|game| {
                            html! {
                                <GameCard 
                                    title={game.title.clone()} 
                                    description={game.description.clone()} 
                                    thumbnail={game.thumbnail.clone()} 
                                    game_id={game.game_id.clone()}
                                />
                            }
                        }).collect()
                    }
                }
            </div>

            <div class="games-footer retro-panel">
                <p class="stats-text">{"GAMES IN DATABASE: "}<span class="highlight-text">{games.len()}</span></p>
                <p class="stats-text">{"SYSTEM STATUS: "}<span class="blink-text">{"ONLINE"}</span></p>
            </div>
        </div>
    }
}