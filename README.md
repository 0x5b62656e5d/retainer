# Retainer

A Github bot written in Rust that deletes messages older than the configured amount of days (no older than 14 days).

## Commands

```
/duration get
Gets the current configured message retention duration

/duration set <duration>
Sets the current configured message retention duration


/channel add <channel>
Adds a channel for the bot to listen to

/channel remove <channel>
Removes a channel for the bot to listen to

/channel list
Gets all the channels the bot is currently listening to

/ping
Pong!
```

## Self-host

### Requirements

Cloud hosting:
- Docker

Local development:
- cargo
- postgres

### Environment variables

Docker hosting:

```
DISCORD_TOKEN=""
DISCORD_CLIENT_ID=""
DATABASE_URL=""
EXPIRY_DAYS=""
```