use std::rc::Rc;

use yew::{prelude::*, Component};
use yewdux::prelude::Dispatch;

use crate::stores::counter_store::CounterStore;

pub enum Msg {
    Store(Rc<CounterStore>),
}

pub struct DisplayCount {
    dispatch: Dispatch<CounterStore>,
}

impl Component for DisplayCount {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let dispatch = Dispatch::<CounterStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        let count = self.dispatch.get().count;

        html! {
            <div>
                <h3>{"Count"}</h3>
                <div>{count}</div>
            </div>
        }
    }
}
