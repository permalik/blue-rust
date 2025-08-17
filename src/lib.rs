use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawLog {
    pub msg_id: String,
    pub service: String,
    pub content: String,
}
