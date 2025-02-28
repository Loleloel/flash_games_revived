use yew::prelude::*;
use crate::components::game_card::{GameCard, GameCardProps};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let featured_games = vec![
        GameCardProps {
            title: "Cyber Invaders X".to_string(),
            description: Some("Battle through waves of digital enemies in this retro-futuristic space shooter".to_string()),
            thumbnail: None,
            game_id: None,
        },
        GameCardProps {
            title: "Neon Runner 2184".to_string(),
            description: Some("Race through a cyberpunk cityscape, dodging obstacles and collecting power-ups".to_string()),
            thumbnail: None,
            game_id: None,
        },
        GameCardProps {
            title: "Digital Dungeon".to_string(),
            description: Some("Navigate through procedurally generated dungeons in this retro roguelike".to_string()),
            thumbnail: None,
            game_id: None,
        },
    ];

    html! {
        <div class="home-container">
            <div class="hero-section">
                <div class="hero-content">
                    <h1 class="glitch-text">{"FLASH GAMES REVIVED"}</h1>
                    <div class="retro-line"></div>
                    <p class="hero-subtitle">{"[ INSERT COIN TO CONTINUE ]"}</p>
                    <p class="hero-description">{"Rediscover the golden age of web gaming,\nNow powered by WebAssembly"}</p>
                </div>
            </div>
            
            <div class="featured-games">
                <h2 class="section-title">{"TODAY'S TOP GAMES"}</h2>
                <div class="game-grid">
                    {
                        featured_games.iter().map(|game| {
                            html! {
                                <GameCard 
                                    title={game.title.clone()} 
                                    description={game.description.clone()} 
                                    thumbnail={game.thumbnail.clone()} 
                                    game_id={game.game_id.clone()}
                                />
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
            
            <div class="about-section retro-panel">
                <h2 class="section-title">{"SYSTEM STATUS"}</h2>
                <div class="status-grid">
                    <div class="status-item">
                        <span class="status-label">{"MISSION"}</span>
                        <p>{"Preserving the legacy of Flash games through WebAssembly technology"}</p>
                    </div>
                    <div class="status-item">
                        <span class="status-label">{"STATUS"}</span>
                        <p class="blink-text">{"ONLINE"}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}