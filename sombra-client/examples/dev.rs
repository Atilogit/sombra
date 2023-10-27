use sombra_client::Client;
use sombra_types::Battletag;

#[tokio::main]
async fn main() {
    let client = Client::new("http://127.0.0.1:8000");
    dbg!(client.search("gazanie").await.unwrap());
    let btag = Battletag::new("gazanie", 2915);
    dbg!(client.profile(&btag).await.unwrap());
    dbg!(client.profile_full(&btag).await.unwrap());
    dbg!(client.overbuff(&btag).await.unwrap());
}
