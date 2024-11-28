mod handle_list;

use actix_web::web;
//use chrono::prelude::*;
//use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Vec<T>,
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(handle_list::get_list_handler);

    conf.service(scope);
}
