use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Counter {
    value: i64,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ClickPlus,
    ClickMinus,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Counter {
            value: 0,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickPlus => {
                self.value += 1;
                true // Indicate that the Component should re-render
            },
            Msg::ClickMinus => {
                self.value -= 1;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let plus_button_text = "Plus";
        let minus_button_text = "Minus";

        html! {
            <div>
                <button class="counter-plus-btn" onclick=&self.link.callback(|_| Msg::ClickPlus)>{ plus_button_text }</button>
                <button class="counter-minus-btn" onclick=&self.link.callback(|_| Msg::ClickMinus)>{ minus_button_text }</button>
                <p>{"value: "} { &self.value }</p>
            </div>
        }
    }
}
