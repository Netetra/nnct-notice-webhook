mod config;
mod discord;
mod nnct_notice;

use config::get_config;
use discord::gen_json;
use nnct_notice::get_notices;
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Client,
};

fn main() {
    const PAGE_URL: &str = "https://www.niihama-nct.ac.jp";
    let config = get_config("./config.toml").unwrap();

    let notices = get_notices(PAGE_URL);
    let json = gen_json(&config, &notices).unwrap();
    println!("{}", json);
    let client = Client::new();
    let _a = client
        .post(config.webhook_url)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(json)
        .send();
}
