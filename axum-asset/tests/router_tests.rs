mod common;

use axum::http::StatusCode;
use axum_asset::Asset;

use self::common::{get, get_body, get_header, get_status, router};

#[derive(Asset)]
#[asset(dir = "tests/static")]
struct StaticAssets;

async fn test_file(path: &str) {
    let app = router::<StaticAssets>();

    let response = get(app, &format!("/static/{path}")).await;

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
async fn test_all_files() {
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
        test_file(file).await;
    }
}
