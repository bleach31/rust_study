use actix_web::{get, App, HttpResponse, HttpServer, ResponseError,web};
use askama::Template;
use thiserror::Error;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

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

    #[error("Failed to get connection")]
    ConncectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError{}//Enumをインプリ？、ResponseErrorのデフォルト実装を使う

#[get("/")] //actix_rtが決めている、それのためのテンプレート
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError>{
    let conn = db.get()?;

    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry{id,text})
    })?;

    let mut entries = Vec::new();

    for row in rows{        // ?はResutを返す関数内でしか使えない。Resut型に対して使う演算子で、
        entries.push(row?); // Result型からデータを取り出す（ここまではunwrapと同じ）、さらにエラーの時はこの関数のエラーとして、この場でリターンする。
    }

    let html = IndexTemplate{entries};
    let response_body = html.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(),actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed!");
    let conn = pool
        .get()
        .expect("Failed!");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    )
    .expect("Failed!");

    HttpServer::new(move || App::new().service(index).data(pool.clone()))
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
