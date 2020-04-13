use web_view::WebView;

pub fn execute_callback(webview: &mut WebView<()>, function_name: &str, data: String) {
    let cmd = format!("{}('{}')", function_name, data);
    webview.eval(&cmd).expect("failed to execute command on webview");
}
