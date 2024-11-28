use crate::handlers;
//use crate::schema::contents::dsl::*;
use actix_web::{get, HttpResponse, Responder};
use std::collections::HashMap;
//use uuid::Uuid;

#[get("/list")]
async fn get_list_handler() -> impl Responder {
    const MESSAGE: &str = "Content List";

    let scores: HashMap<String, String> = HashMap::from([
        ("Id".to_string(), "1".to_string()),
        ("c_name".to_string(), "Genshin Impact".to_string()),
        ("c_link".to_string(), "null".to_string()),
        ("c_number".to_string(), "1".to_string()),
        ("c_short".to_string(), "1".to_string()),
        ("c_sub".to_string(), "1".to_string()),
        ("c_parent".to_string(), "0".to_string()),
        // Add more key-value pairs as needed
    ]);

    /*let mut connection = establish_connection();
    let content = contents
        .load::<Contents>(&mut connection)
        .expect("Error loading contents");*/

    let response_json = &handlers::GenericResponse::<HashMap<String, String>> {
        success: true,
        message: MESSAGE.to_string(),
        data: vec![scores],
    };

    HttpResponse::Ok().json(response_json)
}
