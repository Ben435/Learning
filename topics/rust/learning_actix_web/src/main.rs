use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod services;

static HOST: &str = "localhost";
static PORT: &str = "8080";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let bind_address = format!("{}:{}", HOST, PORT);
    println!("Starting up HttpServer on '{}'", &bind_address);

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/user").configure(services::user::configure))
            .route("/", web::get().to(index))
    })
    .bind(&bind_address)?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello!")
}
