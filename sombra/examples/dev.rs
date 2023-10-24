use futures::future::join_all;
use sombra::{Battletag, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let found = client.search("gazanie").await.unwrap();

    dbg!(&found);

    let assets = client.assets().await.unwrap();

    std::fs::write("assets.txt", format!("{assets:#?}")).unwrap();

    for f in &found {
        dbg!(f.frame.map(|id| assets.get(&id)));
        dbg!(f.namecard.map(|id| assets.get(&id)));
        dbg!(f.portrait.map(|id| assets.get(&id)));
        dbg!(f.title.map(|id| assets.get(&id)));
    }

    let profiles = join_all([
        client.profile(&found[0].battle_tag),
        client.profile(&Battletag::new("Zusor", 2553)),
    ])
    .await;

    dbg!(profiles);
}
