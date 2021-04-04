use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")] // askamaのためのアトリビュート
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

// enum for errors
#[derive(Error, Debug)] //ここの中にEnumの親？スーパークラス的な何か？がある
enum MyError {
    #[error("Failed to render HTML")]   //これはなに？
    MyAskamaError(#[from] askama::Error), //これはなに？　デフォルト実装？＋Askamaのエラー
}

impl ResponseError for MyError{}//Enumをインプリ？、ResponseErrorのデフォルト実装を使う

#[get("/")] //actix_rtが決めている、それのためのテンプレート
async fn index() -> Result<HttpResponse, MyError>{
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });

    let html = IndexTemplate{entries};
    let response_body = html.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(),actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
