use poise::serenity_prelude as serenity;
use dotenv::dotenv;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct Data {}

mod commands;
mod codename_game;
mod database;

pub mod models;
pub mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("TOKEN").expect("missing TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec!  [   commands::age::age(),
                                commands::start_codenames::start_codenames(),
                                commands::join_team::join_team(),
                            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}