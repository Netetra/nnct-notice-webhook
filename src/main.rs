mod config;
mod discord;
mod nnct_notice;

use config::get_config;
use discord::gen_msg;
use nnct_notice::{get_notices, Notice};
use reqwest::{
    blocking::Client,
    header::{HeaderValue, CONTENT_TYPE},
};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const SAVE_FILE_PATH: &str = "./old-notices.json";
const CONFIG_FILE_PATH: &str = "./config.toml";
const PAGE_URL: &str = "https://www.niihama-nct.ac.jp";

fn main() {
    let config = get_config(CONFIG_FILE_PATH).unwrap();
    let mut new_notices = get_notices(PAGE_URL).unwrap();
    new_notices.reverse();

    let path = Path::new(SAVE_FILE_PATH);
    if !path.is_file() {
        save_notices(&new_notices);
    }

    let mut old_notices = read_old_notices();
    save_notices(&new_notices);
    let notices = diff_vec::<Notice>(&mut old_notices, &new_notices);

    //println!("{:?}", notices);

    let messages = gen_msg(&config, notices);
    let client = Client::new();
    for message in messages {
        let json = match message.to_json() {
            Ok(json) => json,
            Err(_) => continue,
        };
        let _ = client
            .post(&config.webhook_url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(json)
            .send();
    }
}

fn diff_vec<T: PartialEq + Clone>(old: &mut Vec<T>, new: &Vec<T>) -> Vec<T> {
    let old_last = &old.pop().unwrap();
    let mut index: usize = 0;
    for i in new.iter() {
        index += 1;
        if i == old_last {
            println!("{}", index);
            break;
        }
        //index += 1;
    }
    let diff = &new[index..];
    return diff.to_vec();
}

fn read_old_notices() -> Vec<Notice> {
    let mut file = File::open(SAVE_FILE_PATH).unwrap();
    let mut old_notices_json = String::new();
    file.read_to_string(&mut old_notices_json).unwrap();
    let old_notices: Vec<Notice> = serde_json::from_str(&old_notices_json).unwrap();
    return old_notices;
}

fn save_notices(notices: &Vec<Notice>) {
    let notices_json = serde_json::to_string_pretty(notices).unwrap();
    let mut file = File::create(SAVE_FILE_PATH).unwrap();
    file.write_all(notices_json.as_bytes()).unwrap();
}
