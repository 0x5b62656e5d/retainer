use crate::{Data, config::ENV};
use entity::{channels, messages};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait};
use serenity::all::{Context as SerenityContext, Message, ReactionType};

pub async fn message_send(
    ctx: &SerenityContext,
    new_message: &Message,
    data: &Data,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if new_message.author.bot {
        return Ok(());
    }

    let channel_id = match new_message.channel(ctx.http.clone()).await?.guild() {
        Some(ch) => ch,
        None => return Ok(()),
    };

    let id: Result<Option<channels::Model>, DbErr> =
        channels::Entity::find_by_id(channel_id.id.to_string())
            .one(&data.postgres.connection())
            .await;

    match id {
        Ok(Some(channel)) => {
            let new_message_model: messages::ActiveModel = messages::ActiveModel {
                message_id: Set(new_message.id.get().to_string()),
                channel_id: Set(channel.channel_id),
                expires_at: Set((chrono::Utc::now() + chrono::Duration::days(ENV.bot.expiry_days as i64)).naive_utc()),
            };

            ctx.http
                .create_reaction(
                    channel_id.id,
                    new_message.id,
                    &ReactionType::Unicode("⏲️".to_string()),
                )
                .await?;

            new_message_model.insert(&data.postgres.connection()).await?;
            return Ok(());
        }
        Ok(None) => {
            return Ok(());
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }
}
