use crate::shared::types::response_type::ResponseStruct;

pub fn map_response<T>(
    status: u32,
    message: String,
    desc: Option<String>,
    data: Option<T>,
) -> ResponseStruct<T> {
    let resp: ResponseStruct<T> = ResponseStruct {
        data: data,
        status: status,
        message: message,
        desc: desc,
    };
    resp
}
