#[macro_use]
extern crate stdweb;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use stdweb::Value;

struct File {
    contents: String,
}

impl File {
    fn new(contents: String) -> Self {
        File { contents: contents }
    }

    fn empty() -> Self {
        File { contents: String::new() }
    }
}

pub struct Model {
    link: ComponentLink<Self>,
    file: File,
}

pub enum Msg {
    OpenFile,
    SetFile(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let set_file_callback = link.callback(|content: String| Msg::SetFile(content));

        let js_set_file_callback = move |value: Value| {
            set_file_callback.emit(
                value
                    .into_string()
                    .expect("unable to parse payload from setfile")
            )
        };

        js! {
            var set_file_callback = @{js_set_file_callback};
            document.addEventListener("setfile", event => set_file_callback(event.detail.contents));
        };

        Model {
            link: link,
            file: File::empty(),
        }
    }

    fn destroy(&mut self) {
        js! {
            set_file_callback.drop();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenFile => {
                js! { external.invoke(JSON.stringify({ msg: "OpenFile" })); }
            },
            Msg::SetFile(contents) => {
                self.file = File::new(contents);
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
                    <div id="editor">{ &self.file.contents }</div>
                </section>
                <footer></footer>
            </div>
        }
    }
}
