extern crate serde;
extern crate toml;
mod load_data;

use yew::prelude::*;


enum Msg {
    AddOne,
    MinusOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    clock_data: load_data::Timeline,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            clock_data: load_data::read_toml().unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            },
            Msg::MinusOne => {
                self.value -= 1;
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let clocks = self.clock_data.create_html_table();
        html! {
            <>
                <main>
                    <div>
                        <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                        <button onclick=self.link.callback(|_| Msg::MinusOne)>{ "-1" }</button>
                        <p>{ self.value }</p>
                    </div>
                    <div>{ clocks }</div>
                </main>
                <footer class="footer">
                    <div>
                        { "copyright 2021- XXXXX: " }
                        <a href="https://yew.rs">{ "Yew" }</a><br/>
                        { "contribute: visit " }
                        <a href="https://github.com/Seigenkousya/kirara_clock">{ "https://github.com/Seigenkousya/kirara_clock" }</a>
                    </div>
                </footer>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
