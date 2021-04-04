use actix_web::{get, App, HttpResponse, HttpServer,ResponseError};
use thiserror::Error;

// enum for errors
#[derive(Error, Debug)] //ここの中にEnumの親？スーパークラス的な何か？がある
enum MyError {}

impl ResponseError for MyError{}//Enumをインプリ？、ResponseErrorのデフォルト実装を使う

#[get("/")]
async fn index() -> Result<HttpResponse, MyError>{
    let response_body = "Hello world!";

    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(),actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
