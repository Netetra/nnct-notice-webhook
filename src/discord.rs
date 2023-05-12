use crate::{config::Config, nnct_notice::Notice};
use serde::Serialize;

#[derive(Serialize)]
pub struct DiscordEmbed {
    title: String,
    description: String,
    url: String,
    color: u32,
}

#[derive(Serialize)]
pub struct DiscordMessage<'a> {
    pub username: &'a str,
    pub avatar_url: &'a str,
    pub content: &'a str,
    pub embeds: Vec<DiscordEmbed>,
}

impl<'a> DiscordMessage<'a> {
    pub fn to_json(self) -> Result<String, serde_json::Error> {
        return serde_json::to_string(&self);
    }
}

pub fn gen_msg(config: &Config, notices: Vec<Notice>) -> Vec<DiscordMessage> {
    return notices.into_iter().map(|notice| DiscordMessage {
        username: &config.username,
        avatar_url: &config.avatar_url,
        content: "",
        embeds: vec![DiscordEmbed {
            title: notice.title,
            description: match notice.description {
                Some(text) => text,
                None => "本文の内容を取得出来ませんでした".to_string()
            },
            url: notice.url,
            color: config.color
        }]
    }).collect::<Vec<_>>();
}
