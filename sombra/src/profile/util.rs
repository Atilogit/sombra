use tl::{HTMLTag, VDom};

use crate::Error;

pub fn url_file(url: &str) -> crate::Result<&str> {
    url.split('/').last().ok_or(Error::Parse)
}

pub fn find_all<'dom>(
    dom: &'dom VDom<'dom>,
    selector: &'dom str,
) -> impl Iterator<Item = &'dom HTMLTag<'dom>> {
    dom.query_selector(selector)
        .expect("invalid selector")
        .filter_map(|n| n.get(dom.parser())?.as_tag())
}

pub fn find_all2<'dom>(
    dom: &'dom VDom<'dom>,
    tag: &'dom HTMLTag<'dom>,
    selector: &'dom str,
) -> impl Iterator<Item = &'dom HTMLTag<'dom>> {
    tag.query_selector(dom.parser(), selector)
        .expect("invalid selector")
        .filter_map(|n| n.get(dom.parser())?.as_tag())
}

pub fn find<'dom>(dom: &'dom VDom<'dom>, selector: &'dom str) -> Option<&'dom HTMLTag<'dom>> {
    dom.query_selector(selector)
        .expect("invalid selector")
        .next()?
        .get(dom.parser())?
        .as_tag()
}

pub fn find2<'dom>(
    dom: &'dom VDom<'dom>,
    tag: &'dom HTMLTag<'dom>,
    selector: &'dom str,
) -> Option<&'dom HTMLTag<'dom>> {
    tag.query_selector(dom.parser(), selector)
        .expect("invalid selector")
        .next()?
        .get(dom.parser())?
        .as_tag()
}

pub fn find_inner_text<'dom>(dom: &'dom VDom<'dom>, selector: &'dom str) -> Option<String> {
    Some(find(dom, selector)?.inner_text(dom.parser()).into())
}

pub fn find_inner_text2<'dom>(
    dom: &'dom VDom<'dom>,
    tag: &'dom HTMLTag<'dom>,
    selector: &'dom str,
) -> Option<String> {
    Some(find2(dom, tag, selector)?.inner_text(dom.parser()).into())
}

pub fn find_attr<'dom>(dom: &'dom VDom<'dom>, selector: &'dom str, attr: &str) -> Option<String> {
    Some(
        find(dom, selector)?
            .attributes()
            .get(attr)??
            .as_utf8_str()
            .into(),
    )
}

pub fn find_attr2<'dom>(
    dom: &'dom VDom<'dom>,
    tag: &'dom HTMLTag<'dom>,
    selector: &'dom str,
    attr: &str,
) -> Option<String> {
    Some(
        find2(dom, tag, selector)?
            .attributes()
            .get(attr)??
            .as_utf8_str()
            .into(),
    )
}
