use stylist::{style, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub message: String,
}

pub struct StructHello {
    pub stylesheet: Style,
}

impl StructHello {
    fn style() -> Style {
        style!(
            r#"
            color: pink;
        "#
        )
        .unwrap()
    }
}

impl Component for StructHello {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stylesheet: Self::style(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1 class={self.stylesheet.clone()}>{&ctx.props().message}</h1>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
