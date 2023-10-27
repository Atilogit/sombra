use sombra_client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("http://127.0.0.1:8000");
    dbg!(client.search("gazanie").await.unwrap());
}
