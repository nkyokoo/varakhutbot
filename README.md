# varakhutbot

A small Discord music bot written in Rust. It joins your voice channel and plays audio from YouTube links, search terms, or files on the machine it runs on.

Built with [serenity](https://github.com/serenity-rs/serenity), [poise](https://github.com/serenity-rs/poise), [songbird](https://github.com/serenity-rs/songbird) and [symphonia](https://github.com/pdeljanov/Symphonia).

## Commands

| Command | What it does |
|---|---|
| `/join` | Joins the voice channel you're currently in |
| `/play <link, search or file>` | Plays a YouTube link, a search (`/play never gonna give you up`), or a local file path |
| `/stop` | Stops the current sound |
| `/leave` | Leaves the voice channel |
| `/age [user]` | Shows when you (or another user) created your Discord account |

Use `/join` before `/play`.

## What you need

- [Rust](https://rustup.rs/)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) on your `PATH`, for links and searches (`winget install yt-dlp` on Windows)
- [ffmpeg](https://ffmpeg.org/), which yt-dlp can use for some formats
- A Discord bot token from the [Developer Portal](https://discord.com/developers/applications)

## Setup

1. Create an application in the Developer Portal and copy the token from **Bot → Reset Token**.
2. Invite the bot with the `bot` **and** `applications.commands` scopes. Without the second one, slash commands won't show up. Give it the **Connect** and **Speak** permissions.
3. Create a `.env` file next to `Cargo.toml`:

   ```
   DISCORD_TOKEN=your_bot_token
   DISCORD_GUILD_ID=your_server_id
   ```

   To get the server ID, turn on Developer Mode in Discord, right-click your server and choose **Copy Server ID**. Commands registered for a single server appear instantly, whereas global ones can take up to an hour.

4. Run it:

   ```
   cargo run --release
   ```

Never commit `.env`. It's already in `.gitignore`.

## Project layout

- `src/main.rs`: startup, configuration and command registration
- `src/commands.rs`: `/age`, `/join` and `/leave`
- `src/player.rs`: `/play` and `/stop`

## Troubleshooting

- **`InvalidAuthentication`**: Discord rejected the token. Reset it and update `.env`. A `DISCORD_TOKEN` already set in your environment takes priority over the file.
- **Slash commands don't appear**: re-invite the bot with the `applications.commands` scope, and check that `DISCORD_GUILD_ID` matches the server you're in.
- **No sound**: check the console for `track error` or `track started playing`, make sure the bot has Connect and Speak, and confirm `yt-dlp` runs from a terminal.
