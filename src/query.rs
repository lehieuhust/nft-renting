#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};
use crate::msg::ContractInfoResponse;
use crate::state::{ContractInfo, CONTRACT_INFO};

pub fn config(deps: Deps) -> StdResult<ContractInfoResponse> {
    let contract_info: ContractInfo = CONTRACT_INFO.load(deps.storage)?;
    Ok(ContractInfoResponse {
        version: contract_info.version,
        name: contract_info.name,
        admin: deps.api.addr_humanize(&contract_info.admin)?,
    })
}