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

    /// Creates an Axum [`Router`] that serves all embedded files.
    ///
    /// Each embedded file is mounted at its relative path (prefixed with `/`). The router automatically handles HTTP
    /// caching:
    ///
    /// - Sets `ETag`, `Last-Modified`, and `Cache-Control` response headers
    /// - Handles `If-None-Match` and `If-Modified-Since` conditional requests
    /// - Returns `304 Not Modified` when the client's cached version is still valid
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use axum::Router;
    /// use axum_asset::Asset;
    ///
    /// #[derive(Asset)]
    /// #[asset(dir = "tests/static")]
    /// struct StaticAssets;
    ///
    /// // Mount assets at /static
    /// let app = Router::new().nest("/static", StaticAssets::router());
    ///
    /// // Files are now accessible at /static/index.html, /static/css/style.css, etc.
    /// ```
    fn router() -> Router {
        let mut router = Router::new();

        for file_name in Self::iter() {
            let file = Self::get(file_name).unwrap();
            let route = format!("/{}", file.path);

            router = router.route(
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

        router
    }
}
