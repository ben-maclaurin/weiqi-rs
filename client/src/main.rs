use yew::prelude::*;

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
        html! {
            <div>
                {"Board"}
            </div>
        }
    }
}

fn main () {
    yew::start_app::<Model>();
}

