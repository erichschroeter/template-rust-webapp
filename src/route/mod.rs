use std::env;


pub mod image_upload;
pub mod images;
pub mod index;
pub mod generate_manifest;
pub mod manifest;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
