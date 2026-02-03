mod common;

use axum::http::StatusCode;
use axum_asset::Asset;

use crate::common::get_with_headers;

use self::common::{get, get_body, get_header, get_status, router};

#[derive(Asset)]
#[asset(dir = "tests/static")]
struct StaticAssets;

async fn test_file_with_if_none_match(path: &str) {
    let app = router::<StaticAssets>();

    let response = get(app.clone(), &format!("/static/{path}")).await;

    let etag = get_header(&response, "etag").unwrap();
    let last_modified = get_header(&response, "last-modified").unwrap();

    let cached_response = get_with_headers(
        app,
        &format!("/static/{path}"),
        vec![("if-none-match", &etag)],
    )
    .await;

    assert_eq!(get_status(&cached_response), StatusCode::NOT_MODIFIED);

    assert_eq!(get_header(&cached_response, "etag"), Some(etag));
    assert_eq!(
        get_header(&cached_response, "last-modified"),
        Some(last_modified)
    );
    assert_eq!(
        get_header(&cached_response, "cache-control").as_deref(),
        Some("no-cache, public")
    );

    let body = get_body(cached_response).await;
    assert!(body.is_empty());
}

async fn test_file_with_incorrect_if_none_match(path: &str) {
    let app = router::<StaticAssets>();

    let response = get_with_headers(
        app,
        &format!("/static/{path}"),
        vec![("if-none-match", "\"incorrect-etag\"")],
    )
    .await;

    let file = StaticAssets::get(path).unwrap();

    assert_eq!(get_status(&response), StatusCode::OK);

    assert_eq!(
        get_header(&response, "etag"),
        Some(format!("\"{}\"", file.metadata.content_hash))
    );
    assert_eq!(
        get_header(&response, "content-type").as_deref(),
        Some(file.metadata.mime_type)
    );
    assert!(get_header(&response, "last-modified").is_some());
    assert_eq!(
        get_header(&response, "content-length"),
        Some(file.metadata.size.to_string())
    );
    assert_eq!(
        get_header(&response, "cache-control").as_deref(),
        Some("no-cache, public")
    );

    let body = get_body(response).await;
    assert_eq!(body, file.contents);
}

async fn test_file_with_if_modified_since(path: &str) {
    let app = router::<StaticAssets>();

    let response = get(app.clone(), &format!("/static/{path}")).await;

    let etag = get_header(&response, "etag").unwrap();
    let last_modified = get_header(&response, "last-modified").unwrap();

    let cached_response = get_with_headers(
        app,
        &format!("/static/{path}"),
        vec![("if-modified-since", &last_modified)],
    )
    .await;

    assert_eq!(get_status(&cached_response), StatusCode::NOT_MODIFIED);

    assert_eq!(get_header(&cached_response, "etag"), Some(etag));
    assert_eq!(
        get_header(&cached_response, "last-modified"),
        Some(last_modified)
    );
    assert_eq!(
        get_header(&cached_response, "cache-control").as_deref(),
        Some("no-cache, public")
    );

    let body = get_body(cached_response).await;
    assert!(body.is_empty());
}

async fn test_file_with_incorrect_if_modified_since(path: &str) {
    let app = router::<StaticAssets>();

    let response = get_with_headers(
        app,
        &format!("/static/{path}"),
        vec![("if-modified-since", "Thu, 01 Jan 1970 00:00:01 GMT")],
    )
    .await;

    let file = StaticAssets::get(path).unwrap();

    assert_eq!(get_status(&response), StatusCode::OK);

    assert_eq!(
        get_header(&response, "etag"),
        Some(format!("\"{}\"", file.metadata.content_hash))
    );
    assert_eq!(
        get_header(&response, "content-type").as_deref(),
        Some(file.metadata.mime_type)
    );
    assert!(get_header(&response, "last-modified").is_some());
    assert_eq!(
        get_header(&response, "content-length"),
        Some(file.metadata.size.to_string())
    );
    assert_eq!(
        get_header(&response, "cache-control").as_deref(),
        Some("no-cache, public")
    );

    let body = get_body(response).await;
    assert_eq!(body, file.contents);
}

async fn test_file_with_if_none_match_and_if_modified_since(path: &str) {
    let app = router::<StaticAssets>();

    let response = get(app.clone(), &format!("/static/{path}")).await;

    let etag = get_header(&response, "etag").unwrap();
    let last_modified = get_header(&response, "last-modified").unwrap();

    let cached_response = get_with_headers(
        app,
        &format!("/static/{path}"),
        vec![
            ("if-modified-since", &last_modified),
            ("if-none-match", &etag),
        ],
    )
    .await;

    assert_eq!(get_status(&cached_response), StatusCode::NOT_MODIFIED);

    assert_eq!(get_header(&cached_response, "etag"), Some(etag));
    assert_eq!(
        get_header(&cached_response, "last-modified"),
        Some(last_modified)
    );
    assert_eq!(
        get_header(&cached_response, "cache-control").as_deref(),
        Some("no-cache, public")
    );

    let body = get_body(cached_response).await;
    assert!(body.is_empty());
}

async fn test_file_with_incorrect_if_none_match_and_if_modified_since(path: &str) {
    let app = router::<StaticAssets>();

    let response = get(app.clone(), &format!("/static/{path}")).await;

    let last_modified = get_header(&response, "last-modified").unwrap();

    let cached_response = get_with_headers(
        app,
        &format!("/static/{path}"),
        vec![
            ("if-modified-since", &last_modified),
            ("if-none-match", "incorrect-etag"),
        ],
    )
    .await;

    let file = StaticAssets::get(path).unwrap();

    assert_eq!(get_status(&cached_response), StatusCode::OK);

    assert_eq!(
        get_header(&cached_response, "etag"),
        Some(format!("\"{}\"", file.metadata.content_hash))
    );
    assert_eq!(
        get_header(&cached_response, "content-type").as_deref(),
        Some(file.metadata.mime_type)
    );
    assert!(get_header(&cached_response, "last-modified").is_some());
    assert_eq!(
        get_header(&cached_response, "content-length"),
        Some(file.metadata.size.to_string())
    );
    assert_eq!(
        get_header(&cached_response, "cache-control").as_deref(),
        Some("no-cache, public")
    );

    let body = get_body(cached_response).await;
    assert_eq!(body, file.contents);
}

async fn test_file_with_if_none_match_and_incorrect_if_modified_since(path: &str) {
    let app = router::<StaticAssets>();

    let response = get(app.clone(), &format!("/static/{path}")).await;

    let etag = get_header(&response, "etag").unwrap();
    let last_modified = get_header(&response, "last-modified").unwrap();

    let cached_response = get_with_headers(
        app,
        &format!("/static/{path}"),
        vec![
            ("if-modified-since", "Thu, 01 Jan 1970 00:00:01 GMT"),
            ("if-none-match", &etag),
        ],
    )
    .await;

    // This'll still return Not Modified because the If-None-Match takes precedence over If-Modified-Since
    assert_eq!(get_status(&cached_response), StatusCode::NOT_MODIFIED);

    assert_eq!(get_header(&cached_response, "etag"), Some(etag));
    assert_eq!(
        get_header(&cached_response, "last-modified"),
        Some(last_modified)
    );
    assert_eq!(
        get_header(&cached_response, "cache-control").as_deref(),
        Some("no-cache, public")
    );

    let body = get_body(cached_response).await;
    assert!(body.is_empty());
}

async fn test_file_with_incorrect_if_none_match_and_incorrect_if_modified_since(path: &str) {
    let app = router::<StaticAssets>();

    let response = get_with_headers(
        app,
        &format!("/static/{path}"),
        vec![
            ("if-modified-since", "Thu, 01 Jan 1970 00:00:01 GMT"),
            ("if-none-match", "\"incorrect-etag\""),
        ],
    )
    .await;

    let file = StaticAssets::get(path).unwrap();

    assert_eq!(get_status(&response), StatusCode::OK);

    assert_eq!(
        get_header(&response, "etag"),
        Some(format!("\"{}\"", file.metadata.content_hash))
    );
    assert_eq!(
        get_header(&response, "content-type").as_deref(),
        Some(file.metadata.mime_type)
    );
    assert!(get_header(&response, "last-modified").is_some());
    assert_eq!(
        get_header(&response, "content-length"),
        Some(file.metadata.size.to_string())
    );
    assert_eq!(
        get_header(&response, "cache-control").as_deref(),
        Some("no-cache, public")
    );

    let body = get_body(response).await;
    assert_eq!(body, file.contents);
}

#[tokio::test]
async fn test_all_files_with_if_none_match() {
    let files = [
        "index.html",
        "empty.txt",
        "data.json",
        "no-extension",
        "script.js",
        "style.css",
        "nested/deep/file.txt",
    ];

    for file in files {
        test_file_with_if_none_match(file).await;
    }
}

#[tokio::test]
async fn test_all_files_with_incorrect_if_none_match() {
    let files = [
        "index.html",
        "empty.txt",
        "data.json",
        "no-extension",
        "script.js",
        "style.css",
        "nested/deep/file.txt",
    ];

    for file in files {
        test_file_with_incorrect_if_none_match(file).await;
    }
}

#[tokio::test]
async fn test_all_files_with_if_modified_since() {
    let files = [
        "index.html",
        "empty.txt",
        "data.json",
        "no-extension",
        "script.js",
        "style.css",
        "nested/deep/file.txt",
    ];

    for file in files {
        test_file_with_if_modified_since(file).await;
    }
}

#[tokio::test]
async fn test_all_files_with_incorrect_if_modified_since() {
    let files = [
        "index.html",
        "empty.txt",
        "data.json",
        "no-extension",
        "script.js",
        "style.css",
        "nested/deep/file.txt",
    ];

    for file in files {
        test_file_with_incorrect_if_modified_since(file).await;
    }
}

#[tokio::test]
async fn test_all_files_with_if_none_match_and_if_modified_since() {
    let files = [
        "index.html",
        "empty.txt",
        "data.json",
        "no-extension",
        "script.js",
        "style.css",
        "nested/deep/file.txt",
    ];

    for file in files {
        test_file_with_if_none_match_and_if_modified_since(file).await;
    }
}

#[tokio::test]
async fn test_all_files_with_incorrect_if_none_match_and_if_modified_since() {
    let files = [
        "index.html",
        "empty.txt",
        "data.json",
        "no-extension",
        "script.js",
        "style.css",
        "nested/deep/file.txt",
    ];

    for file in files {
        test_file_with_incorrect_if_none_match_and_if_modified_since(file).await;
    }
}

#[tokio::test]
async fn test_all_files_with_if_none_match_and_incorrect_if_modified_since() {
    let files = [
        "index.html",
        "empty.txt",
        "data.json",
        "no-extension",
        "script.js",
        "style.css",
        "nested/deep/file.txt",
    ];

    for file in files {
        test_file_with_if_none_match_and_incorrect_if_modified_since(file).await;
    }
}

#[tokio::test]
async fn test_all_files_with_incorrect_if_none_match_and_incorrect_if_modified_since() {
    let files = [
        "index.html",
        "empty.txt",
        "data.json",
        "no-extension",
        "script.js",
        "style.css",
        "nested/deep/file.txt",
    ];

    for file in files {
        test_file_with_incorrect_if_none_match_and_incorrect_if_modified_since(file).await;
    }
}
