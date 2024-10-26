use crate::{codename_game::types, database, Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};

#[poise::command(slash_command, prefix_command)]
pub async fn join_team(
    ctx: Context<'_>,
    team: types::Team
) -> Result<(), Error> {
    let channel_id = ctx.channel_id().get();
    // Update team if player already is in team
    // 
    Ok(())
}