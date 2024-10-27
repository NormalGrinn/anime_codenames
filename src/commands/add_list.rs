use crate::{database, Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};

#[poise::command(slash_command, prefix_command)]
pub async fn add_list(
    ctx: Context<'_>,
    #[description = "Your AL accountname"]
    username: String,
) -> Result<(), Error> {
    let id = ctx.channel_id().get() as i64;
    match database::add_anime_clue(id, &username).await {
        Ok(_) => {
            let message = format!("Added {}'s list to the pool", username);
            ctx.send(CreateReply::default().content(message).ephemeral(true)).await?;
        },
        Err(e) => {
            eprintln!("{:?}", e);
            ctx.send(CreateReply::default().content("Error adding list").ephemeral(true)).await?;
        },
    }
    Ok(())
}