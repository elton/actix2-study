use actix_web::{web, App, Either, Error, HttpRequest, HttpResponse, HttpServer, Responder};

// curl http://localhost:8088/responder/str
async fn responder_str() -> &'static str {
    "responder_str"
}

// curl http://localhost:8088/responder/string
async fn responder_string() -> String {
    "responder_string".to_owned()
}

// curl http://localhost:8088/responder/impl_responder
async fn responder_impl_responder() -> impl Responder {
    web::Bytes::from_static(b"responder_string")
}

use futures::future::{ready, Ready};
use serde::Serialize;

// 自定义 Response
#[derive(Serialize)]
struct ResponseWrapper<T> {
    code: i32,
    msg: String,
    data: Option<T>,
}

// Responder
impl<T> Responder for ResponseWrapper<T>
where
    T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

// curl http://localhost:8088/responder/custom_responder
async fn responder_custom_responder() -> impl Responder {
    ResponseWrapper {
        code: 0,
        msg: "success".to_string(),
        data: Some("custom_responder".to_string()),
    }
}

use futures::future::ok;
use futures::stream::once;

// curl http://localhost:8088/responder/stream
async fn responder_stream_responder() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

// curl http://localhost:8088/responder/either
async fn responder_either_responder() -> RegisterResult {
    Either::A(HttpResponse::BadRequest().body("Bad data"))
    // Either::B(Ok("Hello!"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(
            web::scope("/responder")
                .route("/str", web::get().to(responder_str))
                .route("/string", web::get().to(responder_string))
                .route("/impl_responder", web::get().to(responder_impl_responder))
                .route(
                    "/custom_responder",
                    web::get().to(responder_custom_responder),
                )
                .route("/stream", web::get().to(responder_stream_responder))
                .route("/either", web::get().to(responder_either_responder)),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
