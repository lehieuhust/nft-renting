use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("invalid cw721 hook message")]
    InvalidCw721HookMessage {},

    #[error("NFTNotFound")]
    NFTNotFound {},

    #[error("PriceNotSet")]
    PriceNotSet {},

    #[error("LendedNFT")]
    LendedNFT {},

    #[error("RentedNFT")]
    RentedNFT {},

    #[error("LendTimeTooShort")]
    LendTimeTooShort {},

    #[error("LendTimeTooLong")]
    LendTimeTooLong {},

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
