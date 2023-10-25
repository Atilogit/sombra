use futures::future::join_all;
use sombra::{Battletag, Client};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let client = Client::new();

    let found = client.search("gazanie").await.unwrap();

    let assets = client.assets().await.unwrap();

    std::fs::write("assets.txt", format!("{assets:#?}")).unwrap();

    let profiles = join_all([
        client.profile(found[0].battle_tag.clone()),
        client.profile(Battletag::new("Zusor", 2553)),
        client.profile(Battletag::new("fankk", 21714)),
        client.profile(Battletag::new("Kaynomi", 1965)),
    ])
    .await;

    for profile in profiles {
        match profile {
            Ok(p) => {
                std::fs::write(format!("{:#}.txt", &p.battletag), format!("{p:#?}")).unwrap();
            }
            Err(e) => panic!("{e:#?}"),
        }
    }
}
