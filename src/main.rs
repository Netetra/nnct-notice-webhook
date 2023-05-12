mod config;
mod discord;
mod nnct_notice;

use config::get_config;
use discord::gen_msg;
use nnct_notice::get_notices;
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    blocking::Client,
};

fn main() {
    const PAGE_URL: &str = "https://www.niihama-nct.ac.jp";
    let config = get_config("./config.toml").unwrap();

    let notices = get_notices(PAGE_URL);
    let messages = gen_msg(&config, notices);
    let client = Client::new();
    for message in messages {
        let json = match message.to_json() {
            Ok(json) => json,
            Err(_) => continue
        };
        println!("{}",json);
        client
            .post(&config.webhook_url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(json)
            .send();
    }
}
