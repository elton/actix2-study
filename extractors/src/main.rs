use actix_web::{error, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer};

use serde::Deserialize;

// 提取器 extractors
#[derive(Deserialize, Debug)]
struct QueryInfo {
    username: String,
}

// curl http://localhost:8088/extractor/multiple/p1/p2?username=xiaoming
async fn extractor_multiple(p: web::Path<(String, String)>, q: web::Query<QueryInfo>) -> String {
    format!("p={:?}, q={:?}", p, q)
}

#[derive(Deserialize, Debug)]
struct PathInfo {
    user_id: u32,
    friend: String,
}

// curl http://localhost:8088/extractor/path/123/friend_name
async fn extractor_path(p: web::Path<PathInfo>) -> String {
    format!("path-param={:?}", p)
}

// curl http://localhost:8088/extractor/manual_path/123/friend_name
async fn extractor_manual_path(req: HttpRequest) -> String {
    let friend: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let user_id: i32 = req.match_info().query("user_id").parse().unwrap();

    format!("user_id={}, friend={}", user_id, friend)
}

// curl http://localhost:8088/extractor/query?username=xiaoming
async fn extractor_query(info: web::Query<QueryInfo>) -> String {
    format!("{:?}", info)
}

#[derive(Deserialize, Debug)]
struct JsonInfo {
    username: String,
}

// curl -i -H 'Content-Type: application/json' -d '{"username": "xiaoming"}' -X POST http://localhost:8088/extractor/json
// curl -i -H 'Content-Type: application/json' -d '{"username": 1}' -X POST http://localhost:8088/extractor/json
async fn extractor_json(info: web::Json<JsonInfo>) -> String {
    format!("{}", info.username)
}

#[derive(Deserialize, Debug)]
struct FormData {
    username: String,
}

/// 使用serde提取表单数据
/// 仅当内容类型为*x-www-form-urlencoded*时，才会调用此处理程序
/// 并且请求的内容可以反序列化为FormData结构
// curl -i -H 'Content-Type: application/x-www-form-urlencoded' -d 'username=xiaoming' -X POST http://localhost:8088/extractor/form
async fn extractor_form(form: web::Form<FormData>) -> String {
    format!("{:?}", form)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // 配置 Json Extractor
            .app_data(web::Json::<JsonInfo>::configure(|cfg| {
                cfg.limit(4096).error_handler(|err, _req| {
                    error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                        .into()
                })
            }))
            .service(
                web::scope("/extractor")
                    .route("/multiple/{p1}/{p2}", web::get().to(extractor_multiple))
                    .route("/path/{user_id}/{friend}", web::get().to(extractor_path))
                    .route(
                        "/manual_path/{user_id}/{friend}",
                        web::get().to(extractor_manual_path),
                    )
                    .route("/query", web::get().to(extractor_query))
                    .route("/json", web::post().to(extractor_json))
                    .route("/form", web::post().to(extractor_form)),
            )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
