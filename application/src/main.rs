use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

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

// curl http://localhost:8088/app_state
async fn app_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {}", app_name) // <- response with app_name
}

use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

// curl http://localhost:8088/counter
async fn counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let c = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            // 由于 HttpServer::new 接收的是 App 工厂函数
            // 所以不同线程的 data 不是同一个实例，所以不是进程级别共享数据，而是线程级别的共享数据
            // 线程级别共享，共享的类型不用实现 线程交换安全，.data(T) 只能用于只读
            // 因此只能用于访问只读数据，如全局配置等
            // Application state is shared with all routes and resources within the same scope.
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            // 进程级别共享，共享的类型需要实现 线程交换安全，可用于读写场景，如计数器。通过 .app_data(T) 初始化
            .app_data(c.clone())
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
            // 添加了统一的`scope` url前缀
            .service(web::scope("app").route("index.html", web::get().to(index4)))
            // 只读
            .route("/app_state", web::get().to(app_state))
            .route("/counter", web::get().to(counter))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
