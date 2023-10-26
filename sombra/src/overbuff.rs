use sombra_types::{Battletag, Division, Group, Overbuff, Rank, Role};
use tl::ParserOptions;
use tracing::instrument;

use crate::{
    util::{find2, find_all, find_all2, find_attr2},
    Client, Error,
};

impl Client {
    #[instrument(level = "debug", skip(self))]
    pub async fn overbuff(&self, btag: &Battletag) -> crate::Result<Overbuff> {
        let mut ranks = Vec::new();
        let url = "https://www.overbuff.com/players/";
        let html = self.get(&format!("{url}{btag:#}/")).await?;
        let dom = tl::parse(&html, ParserOptions::new())?;
        let container = find_all(&dom, "div.flex.flex-row.justify-end.gap-x-4")
            .nth(1)
            .ok_or_else(Error::parse)?;

        for rank_container in find_all2(&dom, container, "div.flex") {
            let role_len = find2(&dom, rank_container, "svg")
                .ok_or_else(Error::parse)?
                .inner_html(dom.parser())
                .len();
            let role = match role_len {
                761 => Role::Tank,
                1690 => Role::Damage,
                1535 => Role::Support,
                _ => return Err(Error::parse()),
            };
            let rank_str =
                find_attr2(&dom, rank_container, "img", "alt").ok_or_else(Error::parse)?;
            let split = rank_str.split_once(' ').ok_or_else(Error::parse)?;
            let group = match split.0 {
                "Bronze" => Group::Bronze,
                "Silver" => Group::Silver,
                "Gold" => Group::Gold,
                "Platinum" => Group::Platinum,
                "Diamond" => Group::Diamond,
                "Master" => Group::Master,
                "Grandmaster" => Group::Grandmaster,
                _ => return Err(Error::parse()),
            };
            let division: Division = split.1.parse().map_err(|_| Error::parse())?;
            ranks.push(Rank {
                group,
                division,
                role,
                console: false,
            });
        }

        Ok(Overbuff { ranks })
    }
}
