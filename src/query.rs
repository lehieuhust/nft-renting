#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};
use crate::msg::{ContractInfoResponse, ContractInfo, LendOrderResponse, RentOrderResponse};
use crate::state::{CONTRACT_INFO, LENDED_ITEMS, RENTED_ITEMS};

pub fn config(deps: Deps) -> StdResult<ContractInfoResponse> {
    let contract_info: ContractInfo = CONTRACT_INFO.load(deps.storage)?;
    Ok(ContractInfoResponse {
        version: contract_info.version,
        name: contract_info.name,
        admin: deps.api.addr_humanize(&contract_info.admin)?,
    })
}

pub fn query_lend_order(deps: Deps, token_id: String, cw721_contract: String) -> StdResult<LendOrderResponse> {
    let lend_id = format!("{cw721_contract}{token_id}");
    let lend_order = LENDED_ITEMS.load(deps.storage, &lend_id)?;
    Ok(LendOrderResponse {
        lender: lend_order.lender,
        lend_amount: lend_order.lend_amount,
        lend_time: lend_order.lend_time,
        nft_id: lend_order.nft_id,
        nft_contract: lend_order.nft_contract,
    })
}
pub fn query_rent_order(deps: Deps, token_id: String, cw721_contract: String) -> StdResult<RentOrderResponse> {
    let rent_id = format!("{cw721_contract}{token_id}");
    let rent_order = RENTED_ITEMS.load(deps.storage, &rent_id)?;
    Ok(RentOrderResponse {
        renter: rent_order.renter,
        rent_amount: rent_order.rent_amount,
        expiration: rent_order.expiration,
        nft_id: rent_order.nft_id,
        nft_contract: rent_order.nft_contract,
    })
}