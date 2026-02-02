use axum_asset::Asset;

#[derive(Asset)]
#[asset(dir = "tests/static")]
pub struct StaticAssets;

#[test]
fn test() {}
