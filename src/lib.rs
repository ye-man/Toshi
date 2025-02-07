pub mod cluster;
pub mod commit;
pub mod error;
mod handle;
mod handlers;
pub mod index;
mod query;
mod results;
pub mod router;
pub mod settings;
pub mod shutdown;
pub mod support;
pub mod utils;

pub type Result<T> = std::result::Result<T, crate::error::Error>;
