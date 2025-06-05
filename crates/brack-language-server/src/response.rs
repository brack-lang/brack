use crate::server::Server;
use anyhow::Result;
use serde_json::Value;

impl Server {
    pub(crate) async fn handle_response(&self, _: &Value, _: i64) -> Result<()> {
        Ok(())
    }
}
