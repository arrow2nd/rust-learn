use actix_web::{web, App, HttpRequest, HttpServer};

const SERVER_ADDR: &str = "127.0.0.1:8888";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[Server] http://{}/", SERVER_ADDR);

    // サーバを起動
    HttpServer::new(|| {
        // ルーティング
        App::new().route("/", web::get().to(index))
    })
    // NOTE: '?' ... Result / Option 型から Ok / Some の値を戻す
    //       値が Err の場合は、関数全体から Err を返す (= return Err;)
    //       そのため、Result / Option を返すことができる関数内でのみ使用可能
    // LINK: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    .bind(SERVER_ADDR)?
    .run()
    .await
}

async fn index(req: HttpRequest) -> &'static str {
    println!("request: {:?}", req);
    "Hello, Actix!"
}
