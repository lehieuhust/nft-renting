use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, CanonicalAddr, Uint128};
use cw721::{Cw721ReceiveMsg, Expiration};

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
    pub admin: CanonicalAddr
}

#[cw_serde]
pub struct InstantiateMsg {
    pub name: Option<String>,
    pub version: Option<String>,
    pub admin: Option<Addr>,
}

#[cw_serde]
pub enum Cw721HookMsg {
    /// Locks the token in the contract for the desired time while setting the recipient as the sender if not provided.
    LendNft {
        lend_time: Expiration,
        lend_amount: Uint128,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
    RentNft {
        token_id: String,
        lender: Addr,
    },
    RedeemNft {
        token_id: String,
    },
    EditLendingOrder {
        token_id: String,
    },
    DelistNft {
        token_id: String,
    }
}

#[cw_serde]
pub struct LendDetails {
    /// What the lender offers.
    pub lend_amount: Uint128,
    pub expiration: Expiration,
    pub nft_id: String,
    pub nft_contract: String,
}

#[cw_serde]
pub struct RentDetails {
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

/// https://docs.opensea.io/docs/metadata-standards
/// Replicates OpenSea Metadata Standards
#[cw_serde]
#[derive(Default)]
pub struct TokenExtension {
    /// The name of the token
    pub name: String,
    /// The original publisher of the token
    pub publisher: String,
    /// An optional description of the token
    pub description: Option<String>,
    /// The metadata of the token (if it exists)
    pub attributes: Vec<MetadataAttribute>,
    /// URL to token image
    pub image: String,
    /// Raw SVG image data
    pub image_data: Option<String>,
    /// A URL to the token's source
    pub external_url: Option<String>,
    /// A URL to any multi-media attachments
    pub animation_url: Option<String>,
    /// A URL to a related YouTube videos
    pub youtube_url: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ContractInfoResponse)]
    ContractInfo {},
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
