use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Color {
    White,
    Aqua,
    Pink,
}

impl Color {
    pub fn map_to_css(&self) -> String {
        match &self {
            Color::White => "white".to_owned(),
            Color::Aqua => "aqua".to_owned(),
            Color::Pink => "pink".to_owned(),
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let class = "header title";

    let style_css = style!(
        r#"
            .white {
                color: white;
            }

            .aqua {
                color : aqua;
            }

            .pink {
                color: pink;
            }
        "#
    )
    .ok();

    html! {
        <div class={style_css}>
        <h1 class={class}>{"Hello This Is Main Title:"}</h1>
        <h2 class={props.color.map_to_css()}>{&props.title}</h2>
        </div>
    }
}
