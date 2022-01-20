pub mod crypto;

use std::sync::Arc;
use crate::config::crypto::CryptoService;
use std::time::Duration;
use sqlx::postgres::PgPool;
use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter; 

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
    pub personal_salt: String,
    pub jwt_secret: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
            
        info!("Loading configuation");

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
            .context("loading configuration from environment")
    }

    #[instrument(skip(self))]
    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating Database connection pool.");
        PgPool::builder()
            .connect_timeout(Duration::from_secs(30))
            .build(&self.database_url)
            .await
            .context("creating database connection pool")
    }
    
    #[instrument(skip(self))]
    pub fn hashing(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone()),
            salt: Arc::new(self.personal_salt.clone()),
            jwt_secret: Arc::new(self.jwt_secret.clone()),
        }
    }
}