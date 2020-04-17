extern crate tinyfiledialogs as tfd;

use web_view::{ WebView, WVResult };
use crate::file;
use crate::message::Message;
use crate::rpc;

pub fn handle(webview: &mut WebView<()>, arg: &str) -> WVResult {
    match serde_json::from_str(arg) {
        Ok(message) => {
            match message {
                Message::OpenFile => {
                    println!("OpenFileSelector event invoked");

                    match tfd::open_file_dialog("Open File", "", None) {
                        Some(path) => {
                            rpc::set_file(webview, file::read(path));
                            Ok(())
                        },
                        None => Ok(())
                    }
                },
            }
        }
        Err(error) => {
            println!("Could not match event from webview: {}", error);
            Ok(())
        }
    }
}
