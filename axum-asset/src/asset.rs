use axum::{
    Router,
    http::{HeaderMap, header::IF_NONE_MATCH},
    routing::get,
};
use axum_extra::{
    TypedHeader,
    headers::{IfModifiedSince, IfNoneMatch},
};

use crate::EmbeddedFile;

/// Trait for types that provide access to embedded static assets.
pub trait Asset {
    /// Get an embedded file by path.
    ///
    /// **Note**: The path should be relative to the embedded directory, without a leading slash.
    fn get(path: &str) -> Option<EmbeddedFile>;

    /// Iterate over all embedded files.
    fn iter() -> impl Iterator<Item = &'static str>;

    /// Return the number of embedded files.
    fn len() -> usize;

    /// Check if the asset collection is empty.
    fn is_empty() -> bool {
        Self::len() == 0
    }
}

/// Extension trait for mounting embedded assets onto an Axum router.
///
/// This trait provides a convenient way to serve static assets that have been embedded into the binary at compile
/// time using the [`Asset`] derive macro.
pub trait WithAsset {
    /// Mount all assets from an [`Asset`] implementation under the given URL prefix.
    ///
    /// Each embedded file is registered as a GET route with proper HTTP caching support, including `ETag`,
    /// `Last-Modified`, and `Cache-Control` headers. The handler automatically responds with `304 Not Modified` when
    /// appropriate based on `If-None-Match` and `If-Modified-Since` request headers.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use axum::Router;
    /// use axum_asset::{Asset, WithAsset};
    ///
    /// #[derive(Asset)]
    /// #[asset(dir = "static")]
    /// struct StaticAssets;
    ///
    /// let app = Router::new().with_asset::<StaticAssets>("/static");
    /// ```
    fn with_asset<A>(self, prefix: &str) -> Self
    where
        A: Asset;
}

impl WithAsset for Router {
    fn with_asset<A>(self, prefix: &str) -> Self
    where
        A: Asset,
    {
        let mut this = self;

        for file_name in A::iter() {
            let file = A::get(file_name).unwrap();
            let route = if prefix.starts_with('/') {
                format!("{}{}", prefix, file.route)
            } else {
                format!("/{}{}", prefix, file.route)
            };

            this = this.route(
                &route,
                get({
                    move |headers: HeaderMap, if_none_match: Option<TypedHeader<IfNoneMatch>>, if_modified_since: Option<TypedHeader<IfModifiedSince>>| async move {
                        // Workaround for https://github.com/hyperium/headers/issues/204
                        // IfNoneMatch::decode returns Some even when header is absent
                        let if_none_match = if headers.contains_key(IF_NONE_MATCH) {
                            if_none_match
                        } else {
                            None
                        };
                        crate::util::respond(if_none_match, if_modified_since, file)
                    }
                }),
            );
        }

        this
    }
}
