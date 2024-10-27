use crate::{database, Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};

#[poise::command(slash_command, prefix_command)]
pub async fn start_codenames(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id().get();
    todo!();
    Ok(())
}