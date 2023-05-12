mod config;
mod discord;
mod nnct_notice;

use config::get_config;
use discord::gen_msg;
use nnct_notice::get_notices;
use reqwest::{
    blocking::Client,
    header::{HeaderValue, CONTENT_TYPE},
};

fn main() {
    const PAGE_URL: &str = "https://www.niihama-nct.ac.jp";
    let config = get_config("./config.toml").unwrap();
    let notices = get_notices(PAGE_URL).unwrap();
    let messages = gen_msg(&config, notices);
    let client = Client::new();
    for message in messages {
        let json = match message.to_json() {
            Ok(json) => json,
            Err(_) => continue,
        };
        println!("{}", jsonxf::pretty_print(&json).unwrap());
        let _ = client
            .post(&config.webhook_url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(json)
            .send();
    }
}
