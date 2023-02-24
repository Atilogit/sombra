use sombra_lib::{Battletag, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let profile = client
        .profile(&Battletag::new("gazanie", 2915))
        .await
        .unwrap();
    dbg!(profile);
    let profile = client
        .profile(&Battletag::new("Zusor", 2553))
        .await
        .unwrap();
    dbg!(profile);
}
