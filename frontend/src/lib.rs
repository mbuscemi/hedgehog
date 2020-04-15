#[macro_use]
extern crate stdweb;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
// use stdweb::js_export;

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
                // <script>
                //     js! {
                //         var placeFileContents = @{self.link.callback(|contents| Msg::SetFile(contents))};
                //     }
                // </script>
            </div>
        }
    }
}

// impl Model {
//     #[js_export]
//     fn set_file(&self, contents: String) {
//         self.send_message(Msg::SetFile(contents))
//     }
// }
