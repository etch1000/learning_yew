mod components;

use std::ops::Deref;

use components::atoms::main_title::{Color, MainTitle};
use components::molecules::custom_form::{CustomForm, Data};
use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, Style};
use yew::{prelude::*, ContextProvider};

#[derive(Serialize, Deserialize)]
struct LangName {
    username: String,
    fav_lang: String,
}

#[derive(Clone, Default, PartialEq)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

const CSS_FILE: &str = include_str!("main.css");

#[styled_component]
pub fn App() -> Html {
    let user_state = use_state(User::default);
    let name = "etch1000";
    let lang_name = LangName {
        username: name.to_owned(),
        fav_lang: String::from("Rust Programming Language"),
    };

    log!("My name is :", name);
    log!(serde_json::to_string_pretty(&lang_name).ok());

    let class_p = "paragraph";

    let msg: Option<&str> = None;

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

                <ul class="item-list" title="Crates Used">
                    { to_li(num_list) }
                </ul>
            </div>
        </ContextProvider<User>>
        </>
    }
}

fn to_li<T: std::fmt::Display>(list: Vec<T>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}
