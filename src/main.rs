use web_view::*;

fn main() {
    let html_content = "<html><body><h1>Hello, World!</h1></body></html>";

    web_view::builder()
        .title("Hedgehog")
        .content(Content::Html(html_content))
        .size(1024, 768)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
