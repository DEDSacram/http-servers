#[macro_use] extern crate serde_derive;
use std::io::Result;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder, get, post};
use actix_web::web::Json;

#[derive(Serialize)]
struct MyObj {
    result: String,
}
#[get("/test")]
async fn index() -> Result<Json<MyObj>> {
    Ok(Json(MyObj{result: "Hello World".to_string()}))
}
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body(Json(MyObj{result: "Hello World"}))
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
