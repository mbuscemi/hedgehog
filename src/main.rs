#[macro_use]
extern crate serde_derive;

mod event_handler;
mod file;
mod message;
mod rpc;

use web_view::*;

fn main() {
    web_view::builder()
        .title("Hedgehog")
        .content(Content::Html(include_str!("main.html")))
        .size(1024, 768)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(event_handler::handle)
        .run()
        .unwrap();
}
