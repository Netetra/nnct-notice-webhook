use markup5ever::rcdom::Handle;
use reqwest::blocking::{Client, Response};
use soup::{NodeExt, QueryBuilderExt, Soup};
use std::error::Error;

struct Anker {
    content: String,
    href: String,
}

#[derive(Debug)]
pub struct Notice {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
}

fn get_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let res = client.get(url).send()?;
    let html = res.text()?;
    return Ok(html);
}

fn get_root_element(url: &str) -> Result<Handle, Box<dyn Error>> {
    let html = get_html(url)?;
    let soup = Soup::new(&html);
    let root_element = soup.get_handle();
    return Ok(root_element);
}

fn parse_anker<T: NodeExt + QueryBuilderExt>(anker: T) -> Option<Anker> {
    let content = anker.text();
    let href = anker.get("href")?;
    return Some(Anker { content, href });
}

fn get_notice_ankers(root_element: Handle) -> Vec<Anker> {
    return root_element
        .class("home-main-news-con")
        .find()
        .unwrap()
        .tag("a")
        .find_all()
        .map(|anker| parse_anker(anker))
        .flatten()
        .collect::<Vec<_>>();
}

fn _post_json(url: &str, json: String) -> Response {
    let client = Client::new();
    let response = client.post(url).body(json).send().unwrap();
    return response;
}

fn join_notice_content(content_element: Handle) -> String {
    return content_element
        .tag("p")
        .find_all()
        .map(|element| element.text())
        .collect::<Vec<String>>()
        .join("\n");
}

fn get_notice_content(url: &str) -> Option<String> {
    let root_element = match get_root_element(url) {
        Ok(element) => Some(element),
        Err(_) => None,
    }?;
    let content_element = root_element.class("main-con").find()?;
    let content = join_notice_content(content_element);
    return Some(content);
}

pub fn get_notices(url: &str) -> Vec<Notice> {
    let root_element = get_root_element(url).unwrap();
    let ankers = get_notice_ankers(root_element);
    let notices = ankers
        .into_iter()
        .map(|anker| {
            let content_url = url.to_string() + &anker.href;
            return Notice {
                title: anker.content,
                description: get_notice_content(&content_url),
                url: content_url,
            };
        })
        .collect::<Vec<_>>();
    return notices;
}
