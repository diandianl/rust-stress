use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRPC<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: T
}
