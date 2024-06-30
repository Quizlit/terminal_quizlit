// When you write a test for this, please use httpmock
// example: https://github.com/alexliesenfeld/httpmock/blob/master/tests/examples/json_body_tests.rs
// Once you start mocking structs, use mockall -> https://docs.rs/mockall/0.12.1/mockall/

pub async fn get_json(url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(reqwest::get(url).await?.json::<serde_json::Value>().await?)
}
