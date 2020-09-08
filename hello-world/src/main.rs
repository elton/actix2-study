use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// curl http://localhost:8088/
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

// curl http://localhost:8088/again
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again.")
}

use actix_web::get;

// curl http://localhost:8080/hello
#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// curl http://localhost:8088/app/index.html
async fn index4() -> impl Responder {
    HttpResponse::Ok().body("use scope 'app'.")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
            // 添加了统一的`scope` url前缀
            .service(web::scope("app").route("index.html", web::get().to(index4)))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
