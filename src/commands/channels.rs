use crate::{Context, Error};
use entity::channels;
use poise::serenity_prelude as serenity;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait};
use serenity::ChannelId;

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("add", "remove", "list"),
    description_localized("en-US", "Manage the channels the bot listens to")
)]
pub async fn channel(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Adds a channel for the bot to listen to")
)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Channel to listen to"] channel: ChannelId,
) -> Result<(), Error> {
    let new_channel: channels::ActiveModel = channels::ActiveModel {
        channel_id: Set(channel.to_string()),
    };

    new_channel
        .insert(&ctx.data().postgres.connection())
        .await?;

    let channel_name = channel
        .name(ctx.http())
        .await
        .unwrap_or_else(|_| channel.to_string());

    let response = format!("Channel {} has been added.", channel_name);
    ctx.send(
        poise::CreateReply::default()
            .content(response)
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Removes a channel from the bot's listening list")
)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Channel to remove"] channel: ChannelId,
) -> Result<(), Error> {
    let id: Result<Option<channels::Model>, DbErr> =
        channels::Entity::find_by_id(channel.to_string())
            .one(&ctx.data().postgres.connection())
            .await;

    match id {
        Ok(Some(channel_model)) => {
            channels::Entity::delete_by_id(channel_model.channel_id)
                .exec(&ctx.data().postgres.connection())
                .await?;

            let channel_name = channel
                .name(ctx.http())
                .await
                .unwrap_or_else(|_| channel.to_string());

            let response = format!("Channel {} has been removed.", channel_name);
            ctx.send(
                poise::CreateReply::default()
                    .content(response)
                    .ephemeral(true),
            )
            .await?;

            return Ok(());
        }
        Ok(None) => {
            let response = format!("Channel not found.");
            ctx.send(
                poise::CreateReply::default()
                    .content(response)
                    .ephemeral(true),
            )
            .await?;

            return Ok(());
        }
        Err(e) => {
            let response = format!("Failed to remove channel.");
            ctx.send(
                poise::CreateReply::default()
                    .content(response)
                    .ephemeral(true),
            )
            .await?;

            log::error!("Error removing channel: {:?}", e);

            return Err(Box::new(e));
        }
    }
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Lists all channels the bot is listening to")
)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let channels_list: Vec<channels::Model> = channels::Entity::find()
        .all(&ctx.data().postgres.connection())
        .await?;

    if channels_list.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .content("No channels are currently being listened to.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let mut names: Vec<String> = Vec::new();

    for channels in channels_list {
        names.push(format!("<#{}>", channels.channel_id));
    }

    let response = format!("- {}", names.join("\n- "));

    ctx.send(
        poise::CreateReply::default()
            .content(response)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
