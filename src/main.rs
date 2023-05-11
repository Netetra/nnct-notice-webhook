//mod discord;

use std::error::Error;

use markup5ever::rcdom::Handle;
use reqwest::blocking::{Client, Response};
use soup::{NodeExt, QueryBuilderExt, Soup};

struct Anker {
    content: String,
    href: String,
}

#[derive(Debug)]
struct Notice {
    title: String,
    description: Option<String>,
    url: String,
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

fn get_notices(url: &str) -> Vec<Notice> {
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

fn main() {
    const PAGE_URL: &str = "https://www.niihama-nct.ac.jp";
    let notices = get_notices(PAGE_URL);
    for i in notices {
        println!("{:?}", i);
    }
}

// {
//   "username": "NNCTお知らせ取得🦀",
//   "avatar_url": "https://rustacean.net/assets/rustacean-flat-happy.png",
//   "content": "",
//   "tts": false,
//   "embeds": [
//     {
//       "id": 534308028,
//       "title": "新着お知らせ",
//       "color": 11337983,
//       "fields": []
//     },
//     {
//       "id": 16688219,
//       "title": "【5/9更新・受付開始】奨学制度・授業料免除ページを更新しました。",
//       "description": "【奨学制度と授業料免除】ページを更新しましたので、お知らせします。\n\n就職・就学支援＞奨学制度と授業料免除\n\n＜更新内容（受付開始）＞\n\n・高等教育の修学支援新制度による日本学生支援機構給付型奨学金（予約採用※令和６年度４年進級予定者対象）［5/9更新］\n\n・ニコン奨学金［5/9更新］\n\n・天野工業技術研究所奨学基金［5/9更新］\n\n・令和５年石川県能登地方を震源とする地震にかかる災害救助法適用地域の世帯の学生・生徒に対する給付奨学金家計急変採用及び貸与奨学金緊急採用・応急採用［5/9更新］",
//       "url": "https://www.niihama-nct.ac.jp/2023/05/09/entry-event-25173/",
//       "color": 11337983,
//       "fields": []
//     }
//   ],
//   "components": [],
//   "actions": {}
// }
