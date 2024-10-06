use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct BLSResult<R: Serialize> {
    jsonrpc: String,
    id: i64,
    result: R,
}

impl<R: Serialize> BLSResult<R> {
    pub fn new(id: i64, result: R) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result,
        }
    }
}
