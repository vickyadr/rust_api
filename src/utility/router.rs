use crate::handler::h_list::*;
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(get_list).service(post_list);

    conf.service(scope);
}
