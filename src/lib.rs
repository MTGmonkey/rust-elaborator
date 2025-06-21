use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Id_search_results {
    pub boardgame: Boardgame,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Boardgame {
    pub minplayers: i32,
    pub maxplayers: i32,
    pub playingtime: i32,
    pub minplaytime: i32,
    pub maxplaytime: i32,
    pub age: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Boardgame_overview {
    #[serde(rename = "@objectid")]
    pub objectid: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Keyword_search_results {
    #[serde(rename = "boardgame")]
    pub boardgames: Vec<Boardgame_overview>,
}
