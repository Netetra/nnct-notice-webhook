use crate::{config::Config, nnct_notice::Notice};
use serde::Serialize;

#[derive(Serialize)]
pub struct DiscordEmbed<'a> {
    title: &'a str,
    description: &'a str,
    url: &'a str,
    color: u32,
}

#[derive(Serialize)]
struct DiscordMessage<'a> {
    pub username: &'a str,
    avatar_url: &'a str,
    content: &'a str,
    embeds: Vec<DiscordEmbed<'a>>,
}

pub fn gen_json(config: &Config, notices: &Vec<Notice>) -> Result<String, serde_json::Error> {
    let discord_msg = DiscordMessage {
        username: &config.username,
        avatar_url: &config.avatar_url,
        content: "",
        embeds: notices
            .into_iter()
            .map(|notice| DiscordEmbed {
                title: &notice.title,
                description: match &notice.description {
                    Some(text) => text,
                    None => "本文の内容を取得できませんでした",
                },
                url: &notice.url,
                color: config.color,
            })
            .collect::<Vec<_>>(),
    };
    return serde_json::to_string(&discord_msg);
}
