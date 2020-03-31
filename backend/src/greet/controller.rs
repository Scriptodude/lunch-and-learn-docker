use actix_web::{web, get, HttpResponse, Responder};
use actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN;

use super::models::UserInfo;
use super::dao::users;

#[get("/user/{id}/{name}")]
pub async fn greet_name_id(user: web::Path<(usize, String)>) -> impl Responder {
    HttpResponse::Ok()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(greet_user(&UserInfo{
            id: user.0,
            name: user.1.to_string()
        }))
}

#[get("/name/{name}")]
pub async fn greet_name(user: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(greet_user(&users::get_user_by_name(&user.into_inner())))
}

#[get("/id/{id}")]
pub async fn greet_id(id: web::Path<usize>) -> impl Responder {
    HttpResponse::Ok()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(greet_user(&users::get_user_by_id(id.into_inner())))
}

fn greet_user(user: &UserInfo) -> String {
    return format!("Welcome {}", user)
}