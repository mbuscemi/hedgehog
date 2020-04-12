#[derive(Deserialize)]
#[serde(tag = "msg")]
pub enum Message {
    OpenFileSelector,
}
