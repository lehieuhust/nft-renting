use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, CanonicalAddr};
use cw_storage_plus::{Item, Map};
use cw721::Expiration;

use crate::msg::{LendDetails, RentDetails};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractInfo {
    pub name: String,
    pub version: String,
    pub admin: CanonicalAddr,
}

pub struct UserInfo {
    pub token_id: String,
    pub address: Addr,
    pub expires: Option<Expiration>,
}

pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("config");
pub const LENDED_ITEMS: Map<&str, LendDetails> = Map::new("lended_items");
pub const RENTED_ITEMS: Map<&str, RentDetails> = Map::new("rented_items");
pub const CAN_UNWRAP: Item<bool> = Item::new("can_unwrap");
pub const VALID_DENOM: Item<String> = Item::new("valid_denom");