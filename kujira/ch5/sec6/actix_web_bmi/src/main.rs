use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

const SERVER_ADDR: &str = "127.0.0.1:8888";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[Server] http://{}/", SERVER_ADDR);

    // サーバを起動
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/calc", web::get().to(calc))
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

async fn index(_: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            "<html><body><h1>BMI 計算</h1>\
            <form action='calc'>\
            身長: <input name='height' value='160'><br/>\
            体重: <input name='weight' value='70'><br/>\
            <input type='submit' value='送信'/>\
            </form></body></html>",
        ))
}

// フォームのデータ
#[derive(Serialize, Deserialize, Debug)]
pub struct FormBMI {
    height: f64,
    weight: f64,
}

// BMIを計算
async fn calc(q: web::Query<FormBMI>) -> Result<HttpResponse, Error> {
    println!("{:?}", q);

    let h = q.height / 100.0;
    let bmi = q.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;

    // 結果
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h3>BMI = {:.1}, 乖離率 = {:.0}%</h3>", bmi, per)))
}
