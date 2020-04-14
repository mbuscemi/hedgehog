#[macro_use]
extern crate serde_derive;

mod event_handler;
mod file;
mod inline_code;
mod message;
mod rpc;

use web_view::*;

fn main() {
    web_view::builder()
        .title("Hedgehog")
        .content(Content::Html(inline_code::html()))
        .size(1024, 768)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(event_handler::handle)
        .run()
        .unwrap();
}
