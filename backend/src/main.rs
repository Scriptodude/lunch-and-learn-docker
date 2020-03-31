use actix_web::{HttpServer, App};

mod greet;
mod database;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(greet::configure)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
