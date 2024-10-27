use crate::{database, Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};

#[poise::command(slash_command, prefix_command)]
pub async fn complete_setup(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id().get();
    let res = database::create_game(channel_id.try_into().unwrap());
    match res {
        Ok(_) => {
            ctx.say("Started game succesfully").await?;
        },
        Err(e) => {
            match e {
                diesel::result::Error::DatabaseError(database_error_kind, database_error_information) => {
                    ctx.say("A game already is in progress").await?;
                },
                _ => { ctx.say("Error starting new game").await?; },
            }
        },
    }
    Ok(())
}