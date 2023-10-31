use anyhow::{Ok, Result};
use reqwest;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub async fn request<T>(url: &String) -> Result<T>
where
    T: DeserializeOwned,
{
    let response: Response = reqwest::get(url).await?;
    let response_json: T = response.json().await?;
    Ok(response_json)
}
