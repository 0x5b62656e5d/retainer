use crate::util::cleanup::cleanup as perform_cleanup;
use crate::{Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Triggers a manual cleanup of expired messages")
)]
pub async fn cleanup(ctx: Context<'_>) -> Result<(), Error> {
    perform_cleanup(ctx.data().postgres.connection(), ctx.http()).await;

    ctx.send(
        poise::CreateReply::default()
            .content("Messages have been cleaned up.")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
