//! Embed static assets in your Axum application at compile time.
//!
//! This crate provides a derive macro and extension traits for embedding static files (HTML, CSS, JavaScript, images,
//! etc.) directly into your binary and serving them through Axum routes with proper HTTP caching support.
//!
//! # Features
//!
//! - **Compile-time embedding**: Files are read and embedded during compilation
//! - **HTTP caching**: Automatic `ETag`, `Last-Modified`, and `Cache-Control` headers
//! - **Conditional requests**: Handles `If-None-Match` and `If-Modified-Since` with `304 Not Modified`
//! - **MIME type detection**: Automatically determines content types from file extensions
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use axum::Router;
//! use axum_asset::{Asset, WithAsset};
//!
//! // Define a struct and derive Asset, pointing to your static files directory
//! #[derive(Asset)]
//! #[asset(dir = "static")]
//! struct StaticAssets;
//!
//! // Mount the assets on your router
//! let app = Router::new().with_asset::<StaticAssets>("/static");
//! ```
//!
//! With a directory structure like:
//!
//! ```text
//! my-project/
//! ├── src/
//! │   └── main.rs
//! └── static/
//!     ├── index.html
//!     ├── css/
//!     │   └── style.css
//!     └── js/
//!         └── app.js
//! ```
//!
//! The files will be accessible at:
//! - `/static/index.html`
//! - `/static/css/style.css`
//! - `/static/js/app.js`
//!
//! # The `Asset` Derive Macro
//!
//! The [`Asset`] derive macro reads all files from the specified directory at compile time and generates an
//! implementation of the [`Asset`] trait.
//!
//! ## Attributes
//!
//! - `#[asset(dir = "path")]` - Required. Path to the directory containing assets, relative to the crate's
//!   `Cargo.toml`.
//!
//! # Accessing Files Programmatically
//!
//! You can access embedded files directly:
//!
//! ```rust,ignore
//! use axum_asset::Asset;
//!
//! #[derive(Asset)]
//! #[asset(dir = "static")]
//! struct StaticAssets;
//!
//! // Get a specific file by its relative path
//! if let Some(file) = StaticAssets::get("index.html") {
//!     println!("Route: {}", file.route);
//!     println!("Content length: {}", file.contents.len());
//!     println!("MIME type: {}", file.metadata.mime_type);
//!     println!("ETag: {}", file.metadata.content_hash);
//! }
//!
//! // Iterate over all embedded file paths
//! for path in StaticAssets::iter() {
//!     println!("Embedded: {}", path);
//! }
//! ```

mod asset;
mod file;
mod util;

/// Derive macro for implementing the [`Asset`] trait.
///
/// This macro reads all files from the specified directory at compile time and generates an implementation that
/// embeds them into the binary.
///
/// # Attributes
///
/// - `#[asset(dir = "path")]` - Required. Path to the directory containing assets, relative to the crate's
///   `Cargo.toml`.
///
/// # Example
///
/// ```rust,ignore
/// use axum_asset::Asset;
///
/// #[derive(Asset)]
/// #[asset(dir = "static")]
/// struct StaticAssets;
///
/// // Access files via the generated Asset implementation
/// let file = StaticAssets::get("index.html");
/// ```
pub use axum_asset_derive::Asset;

pub use self::{
    asset::{Asset, WithAsset},
    file::{EmbeddedFile, EmbeddedFileMetadata},
};
