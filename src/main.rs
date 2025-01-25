use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let res = reqwest::get("https://rust-lang.org").await.unwrap();
    println!("Status: {}", res.status());
    println!("Body: {:?}", res.text().await.unwrap());
}
