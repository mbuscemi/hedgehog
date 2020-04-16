#[macro_use]
extern crate stdweb;

// #[macro_use]
extern crate stdweb_derive;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

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

// #[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
// #[reference(instance_of = "Msg")]
pub enum Msg {
    OpenFile,
    SetFile(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {

        let set_file_callback = link.callback(|content: String| Msg::SetFile(content));
        let value = js! {
            var callback = @{set_file_callback};
            return document.addEventListener("set_file", content => callback(content));
        };

        Model {
            link: link,
            file: File::empty(),
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
