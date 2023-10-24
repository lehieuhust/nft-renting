use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, CanonicalAddr, Uint128};
use cw721::{
    Cw721ReceiveMsg,Expiration,
};

/// AssetInfo contract_addr is usually passed from the cw20 hook
/// so we can trust the contract_addr is properly validated.
#[cw_serde]
pub enum AssetInfo {
    Token { contract_addr: Addr },
    NativeToken { denom: String },
}

#[cw_serde]
pub struct Asset {
    pub info: AssetInfo,
    pub amount: Uint128,
}

#[cw_serde]
pub struct ContractInfo {
    pub name: String,
    pub version: String,
    pub admin: CanonicalAddr,
    pub cw721_contract: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub name: Option<String>,
    pub version: Option<String>,
    pub admin: Option<Addr>,
    pub cw721_contract: String,
}

#[cw_serde]
pub enum Cw721HookMsg {
    /// Locks the token in the contract for the desired time while setting the recipient as the sender if not provided.
    LendNft {
        lend_time: u64,
        lend_amount: Uint128,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
    RentNft {
        token_id: String,
        cw721_contract: String,
    },
    EditLendingOrder {
        token_id: String,
        lend_time: u64,
        lend_amount: Uint128,
        cw721_contract: String,
    },
    DelistNft {
        token_id: String,
        cw721_contract: String,
    }
}

#[cw_serde]
pub struct LendDetails {
    /// lender address
    pub lender: String,
    /// What the lender offers.
    pub lend_amount: Uint128,
    /// What the lend order time.
    pub lend_time: u64,
    pub nft_id: String,
    pub nft_contract: String,
}

#[cw_serde]
pub struct RentDetails {
    /// renter address
    pub renter: String,
    /// What the lender offers.
    pub rent_amount: Uint128,
    pub expiration: Expiration,
    pub nft_id: String,
    pub nft_contract: String,
}

#[cw_serde]
pub struct MetadataAttribute {
    /// The key for the attribute
    pub trait_type: String,
    /// The value for the attribute
    pub value: String,
    /// The string used to display the attribute, if none is provided the `key` field can be used
    pub display_type: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ContractInfoResponse)]
    ContractInfo {},
    #[returns(LendOrderResponse)]
    LendOrder { token_id: String, cw721_contract: String, },
    #[returns(RentOrderResponse)]
    RentOrder { token_id: String, cw721_contract: String, },
}

#[cw_serde]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ContractInfoResponse {
    pub name: String,
    pub version: String,
    pub admin: Addr,
}

#[cw_serde]
pub struct LendOrderResponse {
    /// lender address
    pub lender: String,
    /// What the lender offers.
    pub lend_amount: Uint128,
    /// What the lend order time.
    pub lend_time: u64,
    pub nft_id: String,
    pub nft_contract: String,
}

#[cw_serde]
pub struct RentOrderResponse {
    /// renter address
    pub renter: String,
    /// What the renter pay.
    pub rent_amount: Uint128,
    pub expiration: Expiration,
    pub nft_id: String,
    pub nft_contract: String,
}