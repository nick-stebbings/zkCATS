// region:    --- Modules
use serde::Deserialize;
use serde_json::Value;

mod community_rpc;

// endregion: --- Modules

// region:    --- RPC Types

#[derive(Deserialize)]
pub(crate) struct RpcRequest {
    pub(crate) id: Option<Value>,
    pub(crate) method: String,
    pub(crate) params: Option<Value>,
}

#[derive(Deserialize)]
struct ParamsForCreate<D> {
    data: D,
}

#[derive(Deserialize)]
struct ParamsById {
    id: i64,
}

// endregion: --- RPC Types
