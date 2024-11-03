use std::time::Duration;

use crate::{
    infrastructure::client::rest_client::RestClientInfrastructure,
    shared::{
        constants::rest_client_constant::SystemDetail,
        types::{response_type::ResponseStruct, system_type::System},
    },
};

pub async fn get_system_detail(ip: String) -> Result<Option<System>, String> {
    let url = format!(
        "{}{}:{}{}{}",
        SystemDetail::Prefix.to_string(),
        ip,
        SystemDetail::Port as u64,
        SystemDetail::Path.to_string(),
        ip
    );
    log::debug!("Get system detail request url : {}", url);
    let resp: Result<ResponseStruct<System>, String> =
        RestClientInfrastructure::get(url, Duration::from_millis(SystemDetail::Timeout as u64))
            .await;
    match resp {
        Ok(s) => {
            log::debug!("Get system detail response: {:?}", s);
            Ok(s.data)
        }
        Err(e) => Err(e),
    }
}
