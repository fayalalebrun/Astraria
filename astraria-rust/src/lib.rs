pub mod app;
pub mod assets;
pub mod graphics;
pub mod input;
pub mod math;
pub mod physics;
pub mod renderer;
pub mod ui;

pub use app::AstrariaApp;

use anyhow::Result;

#[derive(thiserror::Error, Debug)]
pub enum AstrariaError {
    #[error("Graphics error: {0}")]
    Graphics(String),
    #[error("Physics simulation error: {0}")]
    Physics(String),
    #[error("Asset loading error: {0}")]
    AssetLoading(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type AstrariaResult<T> = Result<T, AstrariaError>;
