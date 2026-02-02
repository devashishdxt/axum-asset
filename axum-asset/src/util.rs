use std::{
    str::FromStr,
    time::{Duration, UNIX_EPOCH},
};

use axum::{
    http::{StatusCode, header::ACCEPT_ENCODING},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{CacheControl, ContentType, ETag, IfModifiedSince, IfNoneMatch, LastModified, Vary},
};

use crate::EmbeddedFile;

/// Generate an ETag header from a content hash.
fn etag(embedded_file: EmbeddedFile) -> Option<TypedHeader<ETag>> {
    let content_hash = embedded_file.metadata.content_hash;
    Some(TypedHeader(
        ETag::from_str(&format!("\"{content_hash}\"")).ok()?,
    ))
}

/// Generate a Last-Modified header from a timestamp.
fn last_modified(embedded_file: EmbeddedFile) -> TypedHeader<LastModified> {
    let last_modified = UNIX_EPOCH + Duration::from_secs(embedded_file.metadata.last_modified);
    TypedHeader(LastModified::from(last_modified))
}

/// Generate a Content-Type header from a file extension.
fn content_type(embedded_file: EmbeddedFile) -> Option<TypedHeader<ContentType>> {
    Some(TypedHeader(
        ContentType::from_str(embedded_file.metadata.mime_type).ok()?,
    ))
}

/// Generate a Cache-Control header with no-cache and public directives.
fn cache_control() -> TypedHeader<CacheControl> {
    TypedHeader(CacheControl::new().with_no_cache().with_public())
}

/// Generate a Not-Modified response with appropriate headers.
fn not_modified_response(embedded_file: EmbeddedFile) -> Response {
    (
        StatusCode::NOT_MODIFIED,
        etag(embedded_file),
        last_modified(embedded_file),
        cache_control(),
        TypedHeader(Vary::from(ACCEPT_ENCODING)),
    )
        .into_response()
}

/// Generate an OK response with appropriate headers.
fn ok_response(embedded_file: EmbeddedFile) -> Response {
    (
        StatusCode::OK,
        etag(embedded_file),
        last_modified(embedded_file),
        cache_control(),
        content_type(embedded_file),
        TypedHeader(Vary::from(ACCEPT_ENCODING)),
        embedded_file.contents,
    )
        .into_response()
}

/// Generate a response with appropriate headers based on the request headers.
pub fn respond(
    if_none_match: Option<TypedHeader<IfNoneMatch>>,
    if_modified_since: Option<TypedHeader<IfModifiedSince>>,
    embedded_file: EmbeddedFile,
) -> Response {
    let etag = etag(embedded_file);

    match (if_none_match, if_modified_since) {
        (Some(TypedHeader(if_none_match)), Some(TypedHeader(if_modified_since))) => {
            if (etag.is_some() && if_none_match.precondition_passes(&etag.unwrap().0))
                || if_modified_since.is_modified(
                    UNIX_EPOCH + Duration::from_secs(embedded_file.metadata.last_modified),
                )
            {
                ok_response(embedded_file)
            } else {
                not_modified_response(embedded_file)
            }
        }
        (Some(TypedHeader(if_none_match)), None) => {
            if etag.is_some() && if_none_match.precondition_passes(&etag.unwrap().0) {
                ok_response(embedded_file)
            } else {
                not_modified_response(embedded_file)
            }
        }
        (None, Some(TypedHeader(if_modified_since))) => {
            if if_modified_since
                .is_modified(UNIX_EPOCH + Duration::from_secs(embedded_file.metadata.last_modified))
            {
                ok_response(embedded_file)
            } else {
                not_modified_response(embedded_file)
            }
        }
        (None, None) => ok_response(embedded_file),
    }
}
