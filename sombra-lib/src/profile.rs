use tl::{HTMLTag, VDom};

use crate::{Error, Group, Rank, Result, Role};

pub fn parse_ranks(html: &VDom) -> Result<Vec<Rank>> {
    let mut ranks = Vec::new();
    for platform in html.get_elements_by_class_name("Profile-playerSummary--rankWrapper") {
        let platform = html.nodes()[platform.get_inner() as usize]
            .as_tag()
            .expect("not a tag");
        let console = platform.attributes().is_class_member("controller-view");
        for role_wrapper in platform
            .query_selector(html.parser(), ".Profile-playerSummary--roleWrapper")
            .expect("invalid selector")
        {
            let role_wrapper = html.nodes()[role_wrapper.get_inner() as usize]
                .as_tag()
                .expect("not a tag");

            let rank_url = std::str::from_utf8(
                query_selector(html, role_wrapper, ".Profile-playerSummary--rank")?
                    .attributes()
                    .get("src")
                    .flatten()
                    .ok_or(Error::parse("Rank url not found"))?
                    .as_bytes(),
            )
            .expect("invalid utf8");

            let role_attr = query_selector(html, role_wrapper, ".Profile-playerSummary--role")?
                .children()
                .all(html.parser())
                .get(0)
                .ok_or(Error::parse("Invalid structure"))?
                .as_tag()
                .ok_or(Error::parse("Invalid structure"))?
                .attributes();

            let role_url = std::str::from_utf8(
                role_attr
                    .get("src")
                    .or(role_attr.get("xlink:href"))
                    .flatten()
                    .ok_or(Error::parse("Role url not found"))?
                    .as_bytes(),
            )
            .expect("invalid utf8");

            let split = url_file_name(rank_url)
                .ok_or(Error::parse("Invalid rank url"))?
                .split_once('-')
                .ok_or(Error::parse("Invalid rank url (group)"))?;
            let group = match split.0 {
                "BronzeTier" => Group::Bronze,
                "SilverTier" => Group::Silver,
                "GoldTier" => Group::Gold,
                "PlatinumTier" => Group::Platinum,
                "DiamondTier" => Group::Diamond,
                "MasterTier" => Group::Master,
                "GrandmasterTier" => Group::Grandmaster,
                _ => return Err(Error::parse("Invalid rank")),
            };
            let tier = split.1[..1]
                .parse()
                .map_err(|_| Error::parse("Invalid rank url (tier)"))?;

            let role = match url_file_name(role_url).ok_or(Error::parse("Invalid role url"))? {
                url if url.starts_with("tank") => Role::Tank,
                url if url.starts_with("offense") => Role::Damage,
                url if url.starts_with("support") => Role::Support,
                _ => return Err(Error::parse("Invalid role")),
            };

            ranks.push(Rank {
                group,
                tier,
                role,
                console,
            })
        }
    }
    Ok(ranks)
}

fn query_selector<'a>(
    html: &'a VDom<'a>,
    tag: &'a HTMLTag<'a>,
    query: &'a str,
) -> Result<&'a HTMLTag<'a>> {
    let handle = tag
        .query_selector(html.parser(), query)
        .expect("invalid selector")
        .next()
        .ok_or(Error::parse("Invalid structure"))?;

    Ok(html.nodes()[handle.get_inner() as usize]
        .as_tag()
        .expect("not a tag"))
}

pub fn url_file_name(url: &str) -> Option<&str> {
    Some(url.rsplit_once('/')?.1)
}

pub fn find_by_class(html: &VDom, class: &str) -> Option<String> {
    html.get_elements_by_class_name(class).next().map(|h| {
        html.nodes()[h.get_inner() as usize]
            .inner_text(html.parser())
            .to_string()
    })
}

pub fn tag_content_by_class<'a>(
    html: &'a VDom<'a>,
    class: &'a str,
    tag: &'a str,
) -> Result<Option<&'a str>> {
    if let Some(h) = html.get_elements_by_class_name(class).next() {
        Ok(Some(
            std::str::from_utf8(
                html.nodes()[h.get_inner() as usize]
                    .as_tag()
                    .expect("not a tag")
                    .attributes()
                    .get(tag)
                    .flatten()
                    .ok_or(Error::parse("Unable to read attribute"))?
                    .as_bytes(),
            )
            .expect("invalid utf8"),
        ))
    } else {
        Ok(None)
    }
}
