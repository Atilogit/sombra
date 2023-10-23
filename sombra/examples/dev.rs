use futures::future::join_all;
use sombra::{Battletag, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let found = client.search("gazanie").await.unwrap();

    dbg!(&found);

    let profiles = join_all([
        client.profile(&found[0].battle_tag),
        client.profile(&Battletag::new("Zusor", 2553)),
    ])
    .await;

    dbg!(profiles);
}
