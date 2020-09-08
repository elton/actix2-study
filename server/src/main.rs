use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let one = HttpServer::new(|| {
        App::new().route(
            "/",
            web::get().to(|| HttpResponse::Ok().body("Keepalive for 75 seconds.")),
        )
    })
    .keep_alive(75); // <- Set keep-alive to 75 seconds

    // let _two = HttpServer::new(|| {
    //     App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    // })
    // .keep_alive(); // <- Use `SO_KEEPALIVE` socket option.

    let _three = HttpServer::new(|| App::new().route("/", web::get().to(|| HttpResponse::Ok())))
        .keep_alive(None); // <- Disable keep-alive

    one.bind("127.0.0.1:8088")?.run().await
}
