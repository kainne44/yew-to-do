use color_eyre::eyre::{Report, Result};
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

struct ItemList {
    item_list: Vec<Item>,
}

impl Default for ItemList {
    fn default() -> Self {
        Self {
            item_list: Vec::new(),
        }
    }
}

impl ItemList {
    fn add_item(&mut self, item: Item) -> &mut Self {
        self.item_list.push(item);
        self
    }

    fn remove_item(&mut self, item: Item) -> Result<Item> {
        let index = self.item_list.iter().position(|x: &Item| {
            x == &Item {
                name: item.name.clone(),
                state: ItemState::Incomplete,
            } || x
                == &Item {
                    name: item.name.clone(),
                    state: ItemState::Complete,
                }
        });
        match index {
            Some(i) => Ok(self.item_list.remove(i)),
            None => Err(Report::msg("Item not found!")),
        }
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    html! {
        <div>
        <button{onclick}>{"+1"}</button>
        <p>{*counter}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
