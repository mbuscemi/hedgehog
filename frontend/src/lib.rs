use yew::{html, Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue };
// use serde::{Serialize, Deserialize};

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    OpenFile,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link: link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenFile => {
                // invoke(open_file_command())
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <header>
                    <button onclick=self.link.callback(|_| Msg::OpenFile)>{ "Open" }</button>
                </header>
                <section id="main-editor">
                    <div id="editor"></div>
                </section>
                <footer></footer>
            </div>
        }
    }
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = external)]
//     fn invoke(payload: JsValue);
// }

// #[derive(Serialize, Deserialize)]
// struct OpenFileCommand {
//     msg: String
// }

// fn open_file_command() -> JsValue {
//     JsValue::from_str(r#"{"msg":"OpenFile"}"#)
// }
