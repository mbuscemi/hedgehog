#[macro_use]
extern crate serde_derive;

use web_view::*;

#[derive(Deserialize)]
#[serde(tag = "msg")]
enum Message {
    OpenFileSelector,
}

fn main() {
    web_view::builder()
        .title("Hedgehog")
        .content(Content::Html(HTML))
        .size(1024, 768)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, arg| {
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
        })
        .run()
        .unwrap();
}

const HTML: &str = r#"
<!doctype html>
<html>
    <body>
        <header>
            <button onclick="external.invoke(JSON.stringify({ msg: 'OpenFileSelector' }))">Open</button>
        </header>
        <section id="main-editor"></section>
        <footer></footer>
    </body>
</html>
"#;
