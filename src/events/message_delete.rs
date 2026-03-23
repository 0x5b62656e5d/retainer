use crate::Data;
use entity::messages;
use sea_orm::EntityTrait;
use serenity::all::{Context as SerenityContext, MessageId};

pub async fn message_delete(
    _ctx: &SerenityContext,
    deleted_message_id: &MessageId,
    data: &Data,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    messages::Entity::delete_by_id(deleted_message_id.to_string())
        .exec(&data.postgres.connection())
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to delete message {}: {}", deleted_message_id, err);
        });

    Ok(())
}
