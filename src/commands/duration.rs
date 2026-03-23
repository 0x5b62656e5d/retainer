use crate::{Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("set", "get"),
    description_localized("en-US", "Gets the duration of message retention")
)]
pub async fn duration(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("set"),
    description_localized("en-US", "Gets the duration of message retention")
)]
pub async fn get(ctx: Context<'_>) -> Result<(), Error> {
    let current_duration = *ctx.data().expiry_days.read().await;

    let response = format!(
        "Current message retention duration is {} days",
        current_duration
    );

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
    description_localized("en-US", "Sets the duration of message retention")
)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Duration for message retention"] duration: i64,
) -> Result<(), Error> {
    if duration < 1 || duration > 14 {
        let response = format!(
            "Duration must be between 1 and 14 (inclusive), got {}",
            duration
        );

        ctx.send(
            poise::CreateReply::default()
                .content(response)
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    {
        let mut lock = ctx.data().expiry_days.write().await;
        *lock = duration;
    }

    let response = format!("Duration set to {} days", duration);

    ctx.send(
        poise::CreateReply::default()
            .content(response)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
