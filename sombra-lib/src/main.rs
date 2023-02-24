use futures::future::join_all;
use sombra_lib::{Battletag, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let profiles = join_all([
        // client.profile(&Battletag::new("gazanie", 2915)),
        // client.profile(&Battletag::new("Zusor", 2553)),
        client.profile(&Battletag::new("Tricky", 12470)),
    ])
    .await;

    dbg!(profiles);
}
