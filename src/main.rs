use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello_world)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[cfg(test)]
mod tests;
