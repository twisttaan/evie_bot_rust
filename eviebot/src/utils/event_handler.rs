use std::{error::Error, sync::Arc};
use twilight_gateway::Event;
use twilight_http::Client as HttpClient;
mod commands;
pub async fn handle_event(
    shard_id: u64,
    event: Event,
    http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!ping" => {
            http.create_message(msg.channel_id)
                .content("Pong!")?
                .exec()
                .await?;
        }
        Event::MessageCreate(msg) if msg.content.starts_with("::") => {
            let command_name = msg.content.split_at(2).1;
            let command = commands::find_command(command_name);
            if let Some(command) = command {
                command.execute(msg, http).await?;
            }
        }

        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        // Other events here...
        _ => {}
    }

    Ok(())
}
