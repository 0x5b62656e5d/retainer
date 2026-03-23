use std::env;
use std::sync::LazyLock;

pub struct Env {
    pub discord: Discord,
    pub postgres: Postgres,
    pub bot: Bot,
}

pub struct Postgres {
    pub url: String,
}

pub struct Discord {
    pub token: String,
    pub client_id: String,
}

pub struct Bot {
    pub expiry_days: i64,
}

impl Env {
    fn get_env(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("Environment variable {} not set", key))
    }

    fn new() -> Self {
        dotenv::dotenv().ok();

        Self {
            discord: Discord {
                token: Self::get_env("DISCORD_TOKEN"),
                client_id: Self::get_env("DISCORD_CLIENT_ID"),
            },
            postgres: Postgres {
                url: Self::get_env("DATABASE_URL"),
            },
            bot: Bot {
                expiry_days: {
                    let days: i64 = Self::get_env("EXPIRY_DAYS").parse().unwrap_or_else(|_| 14);

                    if days < 1 || days > 14 {
                        panic!(
                            "EXPIRY_DAYS must be between 1 and 14 (inclusive), got {}",
                            days
                        );
                    }

                    days
                },
            },
        }
    }
}

pub static ENV: LazyLock<Env> = LazyLock::new(Env::new);
