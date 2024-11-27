use std::time::Duration;

use reqwest;
use serde::{de::DeserializeOwned, ser::Serialize};

pub struct RestClientInfrastructure;

impl RestClientInfrastructure {
    fn build_client(timeout: Duration) -> Result<reqwest::Client, std::string::String> {
        return reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| e.to_string());
    }

    pub async fn get<S>(url: String, timeout: Duration) -> Result<S, String>
    where
        S: DeserializeOwned,
    {
        let client = Self::build_client(timeout)?;
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<S>()
            .await
            .map_err(|e| e.to_string())?;
        return Ok(response);
    }

    pub async fn post<R, S>(url: String, body: R, timeout: Duration) -> Result<S, String>
    where
        S: DeserializeOwned,
        R: Serialize,
    {
        let client = Self::build_client(timeout)?;
        let response = client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<S>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(response)
    }

    pub async fn put<R, S>(url: String, body: R, timeout: Duration) -> Result<S, String>
    where
        S: DeserializeOwned,
        R: Serialize,
    {
        let client = Self::build_client(timeout)?;
        let response = client
            .put(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<S>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(response)
    }

    pub async fn delete<S>(url: String, timeout: Duration) -> Result<S, String>
    where
        S: DeserializeOwned,
    {
        let client = Self::build_client(timeout)?;
        let response = client
            .delete(url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<S>()
            .await
            .map_err(|e| e.to_string())?;
        return Ok(response);
    }
}
