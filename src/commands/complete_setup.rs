use crate::{codename_game::{generate_board::generate_board, generate_image::create_image}, database, Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};
use ::serenity::all::{CreateAttachment, CreateMessage};

#[poise::command(slash_command, prefix_command)]
pub async fn complete_setup(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let id = ctx.channel_id().get() as i64;
    let cards = generate_board(id).expect("Error generating boards");
    create_image(cards);
    let attachement = CreateAttachment::path("././output.png").await?;
    ctx.say("Setup has been completed, the game has now started").await?;
    let builder = CreateMessage::new().add_file(attachement);
    ctx.channel_id().send_message(ctx.http(), builder).await?;
    Ok(())
}