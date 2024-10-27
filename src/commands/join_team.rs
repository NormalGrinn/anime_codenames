use crate::{codename_game::types, database, Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};

#[poise::command(slash_command, prefix_command)]
pub async fn join_team(
    ctx: Context<'_>,
    team: types::Team
) -> Result<(), Error> {
    let id = ctx.channel_id().get() as i64;
    let player_id = ctx.author().id.get();
    let player_name = &ctx.author().name;
    match database::add_player(id, player_id, &player_name, team.clone()) {
        Ok(_) => {
            let message = format!("You joined team {}", team);
            ctx.send(CreateReply::default().content(message).ephemeral(true)).await?;
        },
        Err(_) => {
            ctx.send(CreateReply::default().content("Error adding to team").ephemeral(true)).await?;
        },
    }
    Ok(())
}