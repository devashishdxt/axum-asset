mod asset;
mod file;
mod util;

pub use axum_asset_derive::Asset;

pub use self::{
    asset::{Asset, WithAsset},
    file::{EmbeddedFile, EmbeddedFileMetadata},
};
