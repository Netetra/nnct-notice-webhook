pub struct DiscordEmbed {
    title: String,
    content: String,
    url: String,
}

pub struct DiscordMessage {
    content: String,
    embeds: DiscordEmbed,
}
