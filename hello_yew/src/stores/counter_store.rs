use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Eq)]
pub struct CounterStore {
    pub count: u32,
}
