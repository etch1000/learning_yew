mod components;
mod display_count;
mod increment_count;
mod router;
mod stores;

use components::atoms::main_title::{Color, MainTitle};
use components::molecules::custom_form::{CustomForm, Data};
use components::molecules::struct_counter::StructCounter;
use display_count::DisplayCount;
use gloo::console::log;
use increment_count::IncrementCount;
use router::{switch, Route};
use std::ops::Deref;
use stylist::{yew::styled_component, Style};
use yew::{prelude::*, ContextProvider};
use yew_router::prelude::*;

// #[derive(Serialize, Deserialize)]
// struct LangName {
//     username: String,
//     fav_lang: String,
// }

#[derive(Clone, Default, PartialEq)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

const CSS_FILE: &str = include_str!("main.css");

#[styled_component]
pub fn App() -> Html {
    let user_state = use_state(User::default);
    // let name = "etch1000";
    // let lang_name = LangName {
    //     username: name.to_owned(),
    //     fav_lang: String::from("Rust Programming Language"),
    // };

    // log!("My name is :", name);
    // log!(serde_json::to_string_pretty(&lang_name).ok());

    let class_p = "paragraph";

    let msg: Option<&str> = Some("This website is for learning Yew.rs");

    let num_list = vec![
        "Rust Programming Language",
        "Yew.rs",
        "Trunk",
        "Serde",
        "Stylist",
        "Wasm-Bindgen",
        "Gloo",
        "Web-Sys",
    ];

    let css_file = Style::new(CSS_FILE).unwrap();

    let main_title_load = Callback::from(|message: String| log!(message));

    let custom_form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data: Data| {
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.favorite_language = data.favorite_language;
            user_state.set(user);
        })
    };

    let first_render = use_state(|| true);

    use_effect(move || {
        // This code will run :
        // - First Render
        // - All re-renders
        // If auth token exists and if its our first render
        // get all the users todo tasks
        if *first_render {
            // This only runs on first render
            // This shouldn't run on every state refresh and re-render
            // Do whatever data load to be done on first render
            first_render.set(false);
        }

        || {}
    });

    html! {
        <>
        <ContextProvider<User> context={user_state.deref().clone()}>
            <div class={css_file}>

                <MainTitle title="Yew is Cool" color={Color::Pink} onload={main_title_load} />

                <CustomForm onsubmit={custom_form_submit} />

                if class_p == "paragraph" {
                    <p>{"We are going Full Stack Now"}</p>
                }

                if let Some(message) = msg {
                    <p>{message}</p>
                } else {
                    <p>{"No messages to show"}</p>
                }

                <ul class="item-list" title="Technology & Crates Used">
                    { to_li(num_list) }
                </ul>
            </div>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
            <StructCounter />
            <DisplayCount />
            <IncrementCount />
        </ContextProvider<User>>
        </>
    }
}

fn to_li<T: std::fmt::Display>(list: Vec<T>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}
