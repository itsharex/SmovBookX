
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    code: i32,
    msg: String,
    data: T,
}

impl<T> Response<T> {
    pub fn new(code: i32, data: T, msg: &str) -> Response<T> {
        
        Response {
            code,
            data,
            msg: msg.to_string(),
        }
    }
    pub fn ok(data: T, msg: &str) -> Self {
        Self::new(200, data, msg)
    }
    pub fn err(data: T, msg: &str) -> Self {
        Self::new(300, data, msg)
    }
    pub fn _not_found(data: T) -> Self {
        Self::new(404, data, "未找到数据")
    }
}
