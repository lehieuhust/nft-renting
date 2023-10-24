use cw_storage_plus::{Item, Map};

use crate::msg::{ContractInfo, LendDetails, RentDetails};

pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("config");
pub const LENDED_ITEMS: Map<&str, LendDetails> = Map::new("lended_items");
pub const RENTED_ITEMS: Map<&str, RentDetails> = Map::new("rented_items");

pub const CW721_ADDR: Item<String> = Item::new("cw721_contract_addr");
pub const CAN_UNWRAP: Item<bool> = Item::new("can_unwrap");
pub const VALID_DENOM: Item<String> = Item::new("valid_denom");