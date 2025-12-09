// vendored from khronos
mod asset_requirer;
mod fswrapper;
mod memoryvfs;
mod utils;
mod vfs_navigator;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

pub use asset_requirer::AssetRequirer;
pub use fswrapper::FilesystemWrapper;
pub use memoryvfs::{create_memory_vfs_from_map, create_vfs_from_map};

// Re-export rust-vfs for convenience
pub use vfs;
pub use rust_embed;

#[cfg(test)]
mod tests;