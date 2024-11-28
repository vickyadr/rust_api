use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ReceiverList {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: String,
    pub parrent: Option<String>,
    pub short: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<i32>,
}
