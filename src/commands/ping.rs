use crate::{Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Pong")
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis();
    let created = ctx.created_at().timestamp_millis() as u128;
    let duration = now.saturating_sub(created);

    let response = format!("🏓 Pong!\n\n-# Responded in `{}ms`", duration);
    ctx.send(
        poise::CreateReply::default()
            .content(response)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
