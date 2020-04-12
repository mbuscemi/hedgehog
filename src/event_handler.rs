use web_view::{ WebView, WVResult };
use crate::message::Message;

pub fn handle(_webview: &mut WebView<()>, arg: &str) -> WVResult {
    match serde_json::from_str(arg) {
        Ok(message) => {
            match message {
                Message::OpenFileSelector => {
                    println!("OpenFileSelector event invoked");
                    Ok(())
                },
            }
        }
        Err(error) => {
            println!("Could not match event from webview: {}", error);
            Ok(())
        }
    }
}
