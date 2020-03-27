use actix_web::web;

mod models;
mod dao;
mod controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/hello")
            .service(controller::greet_name_id)
            .service(controller::greet_name)
            .service(controller::greet_id)
    );
}
