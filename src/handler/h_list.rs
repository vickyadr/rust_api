use crate::{
    receiver::r_list::ReceiverList,
    utility::stor::{AppState, GenericResponse},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use std::collections::HashMap;

use crate::models::m_content::Content;

#[get("/list")]
pub async fn get_list(pool: web::Data<AppState>) -> impl Responder {
    let query = sqlx::query_as::<_, Content>(r#"SELECT * FROM contents"#)
        .fetch_all(&pool.db)
        .await;

    if query.is_ok() {
        let response = &GenericResponse::<Vec<Content>> {
            success: true,
            message: "Content List".to_string(),
            data: query.unwrap(),
        };
        return HttpResponse::Ok().json(response);
    }

    let response = &GenericResponse::<Vec<String>> {
        success: false,
        message: "No Content".to_string(),
        data: Vec::new(),
    };

    HttpResponse::InternalServerError().json(response)
}

#[post("/list")]
pub async fn post_list(pool: web::Data<AppState>, body: web::Json<ReceiverList>) -> impl Responder {
    let mut data: HashMap<String, String> = HashMap::new();

    if body.title.is_empty() {
        data.insert("title".to_string(), "Title cant be empty".to_string());
    }

    if body.link.is_empty() {
        data.insert("link".to_string(), "Link cant be empty".to_string());
    }

    if data.len() > 0 {
        let response = &GenericResponse::<HashMap<String, String>> {
            success: false,
            message: "Post error".to_string(),
            data: data,
        };
        return HttpResponse::InternalServerError().json(response);
    }

    let query =
        sqlx::query(r#"INSERT INTO contents (content_title, content_link, content_short, content_parrent, content_sub) VALUES (?, ?, ?, ?, ?)"#)
            .bind(body.title.to_string())
            .bind(body.link.to_string())
            .bind(body.short)
            .bind(body.parrent.to_owned().unwrap_or_default())
            .bind(body.sub.to_owned().unwrap_or_default())
            .execute(&pool.db)
            .await
            .map_err(|e: sqlx::Error| e.to_string());

    if let Err(e) = query {
        //if e.contains("Duplicate entry") {
        //    return HttpResponse::BadRequest();
        //}

        let response = &GenericResponse::<Vec<i32>> {
            success: false,
            message: "Insert data error".to_string(),
            data: Vec::new(),
        };

        return HttpResponse::InternalServerError().json(response);
    }

    if query.is_ok() {
        let response = &GenericResponse::<Vec<i32>> {
            success: true,
            message: "Insert Success".to_string(),
            data: Vec::new(),
        };
        return HttpResponse::Ok().json(response);
    }

    let response = &GenericResponse::<Vec<i32>> {
        success: false,
        message: "Internal Error".to_string(),
        data: Vec::new(),
    };

    return HttpResponse::InternalServerError().json(response);
}
