//! Discord bot built on poise (slash commands) and songbird (voice).
//!
//! Configuration is read from `.env` or the environment:
//! - `DISCORD_TOKEN`: the bot token.
//! - `DISCORD_GUILD_ID`: server to register commands in (instant updates).
//!   If it is not a valid number, commands are registered globally instead.

use poise::serenity_prelude as serenity;
use songbird::SerenityInit;
use songbird::driver::{Scheduler, SchedulerConfig};
use songbird::driver::SchedulerMode;

/// Shared state available to every command through `ctx.data()`.
struct Data {
    /// Reused HTTP client, required by songbird's `YoutubeDl` input.
    http: reqwest::Client,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// General-purpose commands and voice channel join/leave.
mod commands;
/// Audio playback commands.
mod player;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = dotenv::var("DISCORD_TOKEN")?;
    // Required: startup fails if it is missing, even though a non-numeric value falls back to global registration below.
    let guild_id = dotenv::var("DISCORD_GUILD_ID")?;
    // GUILD_VOICE_STATES lets the bot see which voice channel a user is in.
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_VOICE_STATES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::age(), commands::join(), commands::leave(), player::play(), player::stop()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let commands = &framework.options().commands;
                match guild_id.trim().parse::<u64>().ok() {
                    Some(id) => {
                        poise::builtins::register_in_guild(ctx, commands, id.into()).await?
                    }
                    None => poise::builtins::register_globally(ctx, commands).await?,
                }
                Ok(Data {
                    http: reqwest::Client::new(),
                })
            })
        })
        .build();

    // One mixer thread per voice connection (songbird's default packs 16 per thread).
    let mut scheduler_config = SchedulerConfig::default();
    scheduler_config.strategy = SchedulerMode::MaxPerThread(std::num::NonZeroUsize::new(1).unwrap());
    let songbird_config = songbird::Config::default().scheduler(Scheduler::new(scheduler_config));

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird_from_config(songbird_config)
        .await?;
    client.start().await?;
    Ok(())
}