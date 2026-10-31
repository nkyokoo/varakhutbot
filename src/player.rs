//! Audio playback (`/play`, `/stop`) through songbird.
//!
//! Sources are resolved in this order: local file, URL via `yt-dlp`, YouTube search via `yt-dlp`.
//! `yt-dlp` must be installed and on `PATH` for the last two.

use crate::{Context, Error};
use poise::async_trait;
use songbird::events::{Event, EventContext, EventHandler, TrackEvent};
use songbird::input::{File, Input, YoutubeDl};

/// Plays a YouTube link, a search query, or a local audio file in your voice channel
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "YouTube URL, search text, or local file path"]
    #[rest]
    query: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("not in a guild")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("songbird not registered")?;

    let Some(call) = manager.get(guild_id) else {
        ctx.say("I'm not in a voice channel. Use /join first.").await?;
        return Ok(());
    };

    // Local paths are read from the machine running the bot, not the user's machine.
    let query = query.trim().to_string();
    let input: Input = if std::path::Path::new(&query).is_file() {
        File::new(query).into()
    } else if query.starts_with("http://") || query.starts_with("https://") {
        YoutubeDl::new(ctx.data().http.clone(), query).into()
    } else {
        YoutubeDl::new_search(ctx.data().http.clone(), query).into()
    };

    // Defer so Discord does not time out the interaction (3s limit) while yt-dlp resolves the source.
    ctx.defer().await?;
    let handle = call.lock().await.play_input(input);
    let _ = handle.add_event(Event::Track(TrackEvent::Error), TrackErrorLogger);
    let _ = handle.add_event(Event::Track(TrackEvent::Play), TrackStateLogger);
    ctx.say("Playing.").await?;
    Ok(())
}

/// Logs a track's playback state to stderr when it errors (decode failure, `yt-dlp` failure, etc.).
struct TrackErrorLogger;

#[async_trait]
impl EventHandler for TrackErrorLogger {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(tracks) = ctx {
            for (state, _) in *tracks {
                eprintln!("track error: {:?}", state.playing);
            }
        }
        None
    }
}

/// Logs to stderr when a track actually starts playing, to tell source problems from Discord-side ones.
struct TrackStateLogger;

#[async_trait]
impl EventHandler for TrackStateLogger {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        eprintln!("track started playing");
        None
    }
}

/// Stops the current sound
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("not in a guild")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("songbird not registered")?;

    if let Some(call) = manager.get(guild_id) {
        call.lock().await.stop();
    }
    ctx.say("Stopped.").await?;
    Ok(())
}
