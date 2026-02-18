use axum::{
    Router,
    body::{Body, Bytes},
    http::{Request, StatusCode},
    response::Response,
};
use axum_asset::Asset;
use http_body_util::BodyExt;
use tower::ServiceExt;

pub fn router<A: Asset>() -> Router {
    Router::new().nest("/static", A::router())
}

pub async fn get(app: Router, uri: &str) -> Response {
    app.oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[allow(unused)]
pub async fn get_with_headers(app: Router, uri: &str, headers: Vec<(&str, &str)>) -> Response {
    let mut request = Request::builder().uri(uri);
    for (key, value) in headers {
        request = request.header(key, value);
    }
    app.oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

pub fn get_status(response: &Response) -> StatusCode {
    response.status()
}

pub fn get_header(response: &Response, header_name: &str) -> Option<String> {
    response
        .headers()
        .get(header_name)
        .map(|value| value.to_str().unwrap().to_string())
}

pub async fn get_body(response: Response) -> Bytes {
    let body = response.into_body();
    let body = body.collect().await.unwrap();

    body.to_bytes()
}
