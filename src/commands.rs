//! General commands and voice channel membership (`/age`, `/join`, `/leave`).

use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Joins your current voice channel
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    // Scoped so the cached guild reference is dropped before any `.await`.
    let (guild_id, channel_id) = {
        let guild = ctx.guild().ok_or("not in a guild")?;
        let channel = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vs| vs.channel_id);
        (guild.id, channel)
    };

    let Some(channel_id) = channel_id else {
        ctx.say("You need to be in a voice channel first.").await?;
        return Ok(());
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("songbird not registered")?;
    manager.join(guild_id, channel_id).await?;
    ctx.say("Joined your voice channel.").await?;
    Ok(())
}

/// Leaves the voice channel
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("not in a guild")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("songbird not registered")?;
    manager.remove(guild_id).await?;
    ctx.say("Left the voice channel.").await?;
    Ok(())
}


