use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RawLog {
    msg_id: String,
    service: String,
    content: String,
}
