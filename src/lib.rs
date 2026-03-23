use serenity::all::Context as SerenityContext;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod commands;
pub mod config;
pub mod database;
pub mod events;
pub mod util;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone)]
pub struct Data {
    pub postgres: Arc<database::postgres::PostgresService>,
    pub serenity_ctx: Arc<SerenityContext>,
    pub expiry_days: Arc<RwLock<i64>>,
}
