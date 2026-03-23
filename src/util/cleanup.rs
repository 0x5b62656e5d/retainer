use std::sync::Arc;

use chrono::Utc;
use entity::messages;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serenity::all::{ChannelId, Http, MessageId};
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn start_cron_jobs(db: DatabaseConnection, http: Arc<Http>) {
    let scheduler: JobScheduler = JobScheduler::new().await.unwrap_or_else(|_| {
        panic!("Failed to create JobScheduler");
    });

    let job: Job = Job::new_async("0 0 * * * *", move |_, _| {
        let db: DatabaseConnection = db.clone();
        let http = http.clone();

        Box::pin(async move {
            let msgs = messages::Entity::find()
                .filter(messages::Column::ExpiresAt.lt((Utc::now()).fixed_offset()))
                .all(&db)
                .await
                .unwrap_or_else(|err| {
                    panic!("Failed to fetch expired messages: {}", err);
                });

            for msg in msgs {
                let channel_id = msg.channel_id.parse::<u64>().unwrap_or_else(|err| {
                    panic!("Failed to parse channel ID: {}", err);
                });

                let message_id = msg.message_id.parse::<u64>().unwrap_or_else(|err| {
                    panic!("Failed to parse message ID: {}", err);
                });

                let discord_channel = ChannelId::new(channel_id);
                let discord_message_id = MessageId::new(message_id);

                if let Err(err) = http
                    .delete_message(discord_channel, discord_message_id, Some("Cleanup"))
                    .await
                {
                    log::error!(
                        "Failed to delete message {} in channel {}: {}",
                        message_id,
                        channel_id,
                        err
                    );
                }
            }

            messages::Entity::delete_many()
                .filter(messages::Column::ExpiresAt.lt((Utc::now()).fixed_offset()))
                .exec(&db)
                .await
                .unwrap_or_else(|err| {
                    panic!("Failed to delete old messages: {}", err);
                });
        })
    })
    .unwrap_or_else(|err| {
        panic!("Failed to create job: {}", err);
    });

    scheduler.add(job).await.unwrap_or_else(|err| {
        panic!("Failed to add job to scheduler: {}", err);
    });

    scheduler.start().await.unwrap_or_else(|err| {
        panic!("Failed to start scheduler: {}", err);
    });
}
