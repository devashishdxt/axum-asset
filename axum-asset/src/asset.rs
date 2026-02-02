use axum::{Router, routing::get};
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

pub trait WithAsset {
    fn with_asset<A>(self) -> Self
    where
        A: Asset;
}

impl WithAsset for Router {
    fn with_asset<A>(self) -> Self
    where
        A: Asset,
    {
        let mut this = self;

        for file_name in A::iter() {
            let file = A::get(file_name).unwrap();

            this = this.route(
                file.route,
                get({
                    move |if_none_match: Option<TypedHeader<IfNoneMatch>>,
                     if_modified_since: Option<TypedHeader<IfModifiedSince>>| async move {
                        crate::util::respond(if_none_match, if_modified_since, file)
                    }
                }),
            );
        }

        this
    }
}
