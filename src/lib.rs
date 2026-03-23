use std::sync::Arc;

use serenity::all::Context as SerenityContext;

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
}
