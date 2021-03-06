use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    macros::command,
    CommandResult,
};


#[command]
#[description = "Ping the bot"]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

