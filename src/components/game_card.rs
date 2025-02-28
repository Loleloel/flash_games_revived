use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameCardProps {
    pub title: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub game_id: Option<String>,
}

#[function_component(GameCard)]
pub fn game_card(props: &GameCardProps) -> Html {
    let description = props.description.clone().unwrap_or_else(|| "No description available".to_string());
    let thumbnail_style = props.thumbnail.clone().map_or_else(
        || "background: linear-gradient(45deg, #1a1a3a 25%, transparent 25%, transparent 75%, #1a1a3a 75%, #1a1a3a), linear-gradient(45deg, #1a1a3a 25%, transparent 25%, transparent 75%, #1a1a3a 75%, #1a1a3a) 10px 10px; background-color: #141428; background-size: 20px 20px;".to_string(),
        |url| format!("background-image: url({})", url)
    );

    let play_button = if let Some(game_id) = &props.game_id {
        html! {
            <div class="game-actions">
                <a href={format!("/play/{}", game_id)} class="play-button">
                    <span class="button-text">{"PLAY"}</span>
                    <span class="button-scanline"></span>
                </a>
            </div>
        }
    } else {
        html! {
            <div class="game-actions">
                <span class="coming-soon">
                    <span class="status-text">{"LOADING..."}</span>
                    <span class="button-scanline"></span>
                </span>
            </div>
        }
    };

    html! {
        <div class="game-card retro-panel">
            <div class="game-thumbnail" style={thumbnail_style}>
                <div class="overlay">
                    <div class="scan-line"></div>
                    <div class="glow"></div>
                </div>
            </div>
            <div class="game-info">
                <h3 class="game-title">{&props.title}</h3>
                <div class="game-description">{description}</div>
                {play_button}
            </div>
        </div>
    }
}