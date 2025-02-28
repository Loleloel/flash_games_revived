use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    pub on_toggle: Callback<()>,
    pub is_dark: bool,
}

#[function_component(ThemeToggle)]
pub fn theme_toggle(props: &ThemeToggleProps) -> Html {
    let onclick = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_| {
            on_toggle.emit(());
        })
    };

    html! {
        <button
            onclick={onclick}
            class="theme-toggle"
            title={if props.is_dark { "Switch to light mode" } else { "Switch to dark mode" }}
            aria-label="Toggle dark mode"
        >
            if props.is_dark {
                {"🌞"}
            } else {
                {"🌙"}
            }
        </button>
    }
}