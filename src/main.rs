use log;
use migration::{Migrator, MigratorTrait};
use poise::{Framework, FrameworkContext};
use retainer::{Data, config::ENV, util::cleanup::start_cron_jobs};
use serenity::{
    Client,
    all::{Context as SerenityContext, FullEvent, GatewayIntents},
};
use std::sync::Arc;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

async fn handle_event(
    ctx: &SerenityContext,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, retainer::Error>,
    data: &Data,
) -> Result<(), retainer::Error> {
    match event {
        FullEvent::Message { new_message } => {
            match retainer::events::message_send::message_send(ctx, new_message, data).await {
                Ok(_) => {}
                Err(err) => {
                    log::error!("Error handling message send: {:?}", err);
                }
            }
        }
        FullEvent::MessageDelete {
            channel_id: _,
            deleted_message_id,
            guild_id: _,
        } => {
            match retainer::events::message_delete::message_delete(ctx, deleted_message_id, data)
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    log::error!("Error handling message delete: {:?}", err);
                }
            }
        }
        _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .add_directive("tracing::span=warn".parse()?)
        .add_directive("serenity::http::ratelimiting=info".parse()?)
        .add_directive("serenity::http::request=info".parse()?);

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_ansi(true)
        .init();

    log::info!("Launching Retainer...");

    log::info!("New messages will be deleted after {} days", ENV.bot.expiry_days);

    let postgres =
        Arc::new(retainer::database::postgres::PostgresService::new(&ENV.postgres.url).await);

    Migrator::up(&postgres.connection(), None).await?;

    log::info!("Migrated successfully!");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS;

    let framework = Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                retainer::commands::ping::ping(),
                retainer::commands::channels::channel(),
                retainer::commands::channels::add(),
                retainer::commands::channels::remove(),
                retainer::commands::channels::list(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(handle_event(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup({
            let postgres = Arc::clone(&postgres);

            move |ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    log::info!("Registered commands globally");

                    let data = Arc::new(Data {
                        postgres,
                        serenity_ctx: Arc::new(ctx.clone()),
                    });

                    Ok(Arc::try_unwrap(data).unwrap_or_else(|arc| (*arc).clone()))
                })
            }
        })
        .build();

    let mut client: Client = Client::builder(&ENV.discord.token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    let http = client.http.clone();

    log::info!("Created cron job");

    start_cron_jobs(postgres.connection(), http).await;

    if let Err(err) = client.start().await {
        log::error!("Client error: {:?}", err);
    }

    Ok(())
}
