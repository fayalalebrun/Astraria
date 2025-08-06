pub mod app;
pub mod assets;
pub mod generated_shaders;
pub mod graphics;
pub mod input;
pub mod math;
pub mod physics;
pub mod renderer;
pub mod scenario;
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
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
}

pub type AstrariaResult<T> = Result<T, AstrariaError>;
