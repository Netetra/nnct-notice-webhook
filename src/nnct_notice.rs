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

fn get_element_from_class(parent_element: &Handle, class: &str) -> Option<Handle> {
    let element = parent_element.class(class).find()?.get_handle();
    return Some(element)
}

fn get_element_all_from_tag(parent_element: &Handle, tag: &str) -> Vec<Handle> {
    return parent_element.tag(tag).find_all()
        .map(|element| element.get_handle()).collect::<Vec<_>>();
}

// fn get_notice_section(root_element: Handle) -> Option<Handle> {
//     let section = root_element
//         .class("home-main-news-con")
//         .find()?
//         .get_handle();
//     return Some(section);
// }

fn parse_anker(anker: Handle) -> Option<Anker> {
    let content = anker.text();
    let href = anker.get("href")?;
    return Some(Anker { content, href });
}

// fn get_ankers(section: &Handle) -> Vec<Anker> {
//     return section
//         .tag("a")
//         .find_all()
//         .map(|anker| parse_anker(anker))
//         .flatten()
//         .collect::<Vec<_>>();
// }

fn join_content(content_element: &Handle) -> String {
    return content_element
        .tag("p")
        .find_all()
        .map(|element| element.text())
        .collect::<Vec<String>>()
        .join("\n");
}

fn get_content(root_element: &Handle) -> Option<String> {
    let content_element = root_element.class("main-con").find()?;
    let content = join_content(&content_element);
    return Some(content);
}

pub fn get_notices(url: &str) -> Result<Vec<Notice>, Box<dyn Error>> {
    let root_element = get_root_element(url)?;
    let notice_section = get_element_from_class(&root_element, "home-main-news-con");
    let ankers = get_element_all_from_tag(&notice_section, "a")
        .into_iter().map(|anker| parse_anker(anker))
        .flatten().collect::<Vec<_>>();
    let notices = ankers
        .into_iter()
        .map(|anker| {
            let content_url = url.to_string() + &anker.href;
            let content_root_element = match get_root_element(&content_url) {
                Ok(element) => element,
                Err(_) => return None
            };
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
