use std::env;

pub mod script;
pub mod image_upload;
pub mod images;
pub mod index;
pub mod manifest;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
