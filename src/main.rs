use color_eyre::eyre::{Report, Result};
use std::fmt::Display;
use yew::prelude::*;

#[derive(PartialEq)]
enum ItemState {
    Incomplete,
    Complete,
}

#[derive(PartialEq)]
struct Item {
    name: String,
    state: ItemState,
}

impl Item {
    fn new(name: String) -> Self {
        Self {
            name,
            state: ItemState::Incomplete,
        }
    }

    fn toggle_complete(&mut self) {
        match self.state {
            ItemState::Complete => self.state = ItemState::Incomplete,
            ItemState::Incomplete => self.state = ItemState::Complete,
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: String::from("To Do Item"),
            state: ItemState::Incomplete,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[function_component]
fn Jim() -> Html {
    let item1 = Item::new(String::from("Tomatoes"));
    let item2 = Item::new(String::from("Gummy Bears"));
    let item3 = Item::new(String::from("Pasta"));
    let list = vec![item1, item2, item3];

    html! {
            <p>{
                for list.into_iter().map(|item| {
                    html_nested!{
                        <p>{ item.name }</p>
                    }
                })
            }</p>

    }

    // html! {
    //     <p>{list.into_iter().collect::<Html>()}</p>
    // }
}

fn main() {
    yew::Renderer::<Jim>::new().render();
}
