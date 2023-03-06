use actix_web::{get,post,web::{self, Data},App,HttpResponse,HttpServer,Responder};
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let counter: Data<AppStateWithCounter> = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
       App::new()
            .app_data(counter.clone())
            .route("/",web::get().to(index2))
    })
    .bind(("127.0.0.1",8000))?
    .run()
    .await
}