use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
// use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TransferInfo {
    pub recipient: Addr,
    pub native: Vec<Coin>,
}
