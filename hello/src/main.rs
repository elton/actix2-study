use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

#[get("/hello")]
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hey, there again!")
}

// 该宏在actix运行时中执行标记的异步函数。该宏可以标记并执行任何异步函数。
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().service(index2));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
