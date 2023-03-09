use actix_web::{get,post,web::{self, Data, ServiceConfig},App,HttpResponse,HttpServer,Responder, guard};
use std::sync::Mutex;

struct AppState {
    app_name:String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[post("/echo")]
async fn echo(req_body:String) -> impl Responder {
    return HttpResponse::Ok().body(req_body);
}

async fn manual_hello() -> impl Responder {
    return HttpResponse::Ok().body("hey There");
}

async fn index2(data: web::Data<AppStateWithCounter>) -> String{
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number:{counter}")
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async {HttpResponse::Ok().body("test")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed)), 
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async {HttpResponse::Ok().body("app")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed)), 
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // let counter: Data<AppStateWithCounter> = web::Data::new(AppStateWithCounter {
    //     counter: Mutex::new(0),
    // });

    // HttpServer::new(move || {
    //    App::new()
    //         .app_data(counter.clone())
    //         .route("/",web::get().to(index2))
    // })
    // .bind(("127.0.0.1",8000))?
    // .run()
    // .await

   HttpServer::new(|| {
    App::new()
        .configure(config)
        .service(web::scope("/api").configure(scoped_config))
        .route(
            "/",
            web::get().to(|| async {HttpResponse::Ok().body("/")}),         
        )
   })
   .bind(("127.0.0.1",8080))?
   .run()
   .await
}