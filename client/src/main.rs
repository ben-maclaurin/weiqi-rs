use yew::prelude::*;
use engine::game::{Board, State};
use engine::chain::Chain;
use std::collections::HashMap;
use engine::mov::{Stone, Move};


struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut b = Board {
            board_states: HashMap::new(),
            size: 9,
            chains: Vec::<Chain>::new(),
        };

        let m = Move {
            intersection: (1, 1),
            stone: Stone::Black,
        };

        &b.update(&m);

        html! {
            <div>
                { for b.board_states.iter().map(|state| self.render_state(state.1)) }
            </div>
        }
    }
}

impl Model {
    fn render_state(&self, state: &State) -> Html {
        let s = match state {
            State::Stone(Stone::Black) => "B",
            State::Stone(Stone::White) => "W",
            _ => "V",
        };

        html! {
            <p>{s}</p>
        }
    }
}

fn main () {
    yew::start_app::<Model>();
}

