// ping command
@command(
    "ping",
    desc="Pong!",
    usage="ping",
    aliases=["pong"],
    min_args=0,
    max_args=0,
)
pub fn on_message(ctx: Context, msg: Message) {
    if msg.content == "!ping" {
        let _ = http
            .create_message(msg.channel_id)
            .content("Pong!")?
            .exec()
            .await?;
    }
}
