use actix_web::{HttpServer, App};

mod greet;
mod database;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(greet::configure)
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
