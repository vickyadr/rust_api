use crate::schema::contents;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Contents {
    pub id: i32,
    pub content_title: String,
    pub content_link: String,
    pub content_short: i8,
    pub content_number: i32,
    pub content_parrent: i32,
    pub content_sub: i32,
}
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "contents"]
pub struct NewContents {
    pub content_title: String,
    pub content_link: String,
    pub content_short: i8,
    pub content_number: i32,
    pub content_parrent: i32,
    pub content_sub: i32,
}
#[derive(Deserialize, AsChangeset)]
#[table_name = "contents"]
pub struct UpdateContents {
    content_title: Option<String>,
    content_link: Option<String>,
    content_short: Option<i8>,
    content_number: Option<i32>,
    content_parrent: Option<i32>,
    content_sub: Option<i32>,
}
