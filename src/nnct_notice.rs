use markup5ever::rcdom::Handle;
use reqwest::blocking::Client;
use soup::{NodeExt, QueryBuilderExt, Soup};
use std::error::Error;

struct Anker {
    content: String,
    href: String,
}

pub struct Notice {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
}

fn get_root_element(url: &str) -> Result<Handle, Box<dyn Error>> {
    let client = Client::new();
    let res = client.get(url).send()?;
    let html = res.text()?;
    let soup = Soup::new(&html);
    let root_element = soup.get_handle();
    return Ok(root_element);
}

fn get_element_from_class(parent_element: &Handle, class: &str) -> Option<Handle> {
    let element = parent_element.class(class).find()?.get_handle();
    return Some(element);
}

fn get_element_all_from_tag(parent_element: &Handle, tag: &str) -> Vec<Handle> {
    return parent_element
        .tag(tag)
        .find_all()
        .map(|element| element.get_handle())
        .collect::<Vec<_>>();
}

fn get_notice_section(parent_element: &Handle) -> Option<Handle> {
    return get_element_from_class(parent_element, "home-main-news-con");
}

fn get_ankers(parent_element: &Handle) -> Vec<Handle> {
    return get_element_all_from_tag(parent_element, "a");
}

fn parse_anker(anker_element: &Handle) -> Result<Anker, String> {
    let content = anker_element.text();
    let href = anker_element
        .get("href")
        .ok_or("anker href is void".to_string())?;
    return Ok(Anker { content, href });
}

fn get_content(parent_element: &Handle) -> Option<String> {
    let content_element = get_element_from_class(parent_element, "main-con")?;
    let content_p_element = get_element_all_from_tag(&content_element, "p");
    let content = join_content_p(&content_p_element);
    return Some(content);
}

fn join_content_p(parent_element: &Vec<Handle>) -> String {
    return parent_element
        .into_iter()
        .map(|element| element.text())
        .collect::<Vec<String>>()
        .join("\n");
}

pub fn get_notices(url: &str) -> Result<Vec<Notice>, Box<dyn Error>> {
    let root_element = get_root_element(url)?;
    let notice_section_element =
        get_notice_section(&root_element).expect("cannot get notice section.");
    let ankers = get_ankers(&notice_section_element)
        .into_iter()
        .map(|anker| parse_anker(&anker))
        .flatten()
        .collect::<Vec<_>>();
    let notices = ankers
        .into_iter()
        .map(|anker| -> Option<Notice> {
            let content_url = url.to_string() + &anker.href;
            let content_root_element = get_root_element(&content_url).ok()?;
            return Some(Notice {
                title: anker.content,
                description: get_content(&content_root_element),
                url: content_url,
            });
        })
        .flatten()
        .collect::<Vec<_>>();
    return Ok(notices);
}
