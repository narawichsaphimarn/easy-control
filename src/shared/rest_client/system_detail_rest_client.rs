use std::time::Duration;

use crate::{
    infrastructure::client::rest_client::RestClientInfrastructure,
    shared::{constants::rest_client_constant::SystemDetail, types::system_type::System},
};

#[tokio::main]
pub async fn get_system_detail(ip: String) -> Option<System> {
    let url = format!(
        "{}{}:{}{}{}",
        SystemDetail::Prefix.to_string(),
        ip,
        SystemDetail::Port.to_string(),
        SystemDetail::Path.to_string(),
        ip
    );
    let resp: Result<System, String> =
        RestClientInfrastructure::get(url, Duration::from_secs(SystemDetail::Timeout as u64)).await;
    match resp {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}
