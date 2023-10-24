use tl::{Node, ParserOptions, VDom};

use crate::{Battletag, Client};

use std::fmt::{Debug, Display};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfile {
    pub title: Option<String>,
    pub endorsement: Endorsement,
    pub portrait: Option<String>,
    pub ranks: Vec<Rank>,
}

bounded_integer::bounded_integer! {
    pub struct Endorsement{ 1..=5 }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub group: Group,
    pub division: Division,
    pub role: Role,
    pub console: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Tank,
    Damage,
    Support,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Group {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    Grandmaster,
}

bounded_integer::bounded_integer! {
    pub struct Division{ 1..=5 }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.group, self.division)
    }
}

impl Client {
    pub async fn profile(&self, btag: &Battletag) -> crate::Result<PlayerProfile> {
        let url = "https://overwatch.blizzard.com/en-us/career/";
        let html = self.get(&format!("{url}{btag:#}/")).await?;
        let dom = tl::parse(&html, ParserOptions::new().track_classes())?;

        let title = find(&dom, ".Profile-player--title")
            .ok_or_else(|| crate::Error::Parse)?
            .inner_text(dom.parser())
            .to_string();

        todo!()
    }
}

fn find<'dom>(dom: &'dom VDom<'dom>, selector: &'dom str) -> Option<&'dom Node<'dom>> {
    dom.query_selector(selector)?.next()?.get(dom.parser())
}
