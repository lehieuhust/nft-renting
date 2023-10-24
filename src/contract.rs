#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::handle::{delist_nft, edit_lend_order, handle_receive_cw721, rent_nft};
use crate::msg::{ContractInfo, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query;
use crate::state::{CONTRACT_INFO, CW721_ADDR};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:nft-renting";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let creator = deps.api.addr_canonicalize(info.sender.as_str())?;
    let config = ContractInfo {
        name: msg.name.unwrap_or(CONTRACT_NAME.to_string()),
        version: msg.version.unwrap_or(CONTRACT_VERSION.to_string()),
        // admin should be multisig
        admin: if let Some(admin) = msg.admin {
            deps.api.addr_canonicalize(admin.as_str())?
        } else {
            creator
        },
        cw721_contract: msg.cw721_contract,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONTRACT_INFO.save(deps.storage, &config)?;
    CW721_ADDR.save(deps.storage, &config.cw721_contract)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("name", config.name.to_string())
        .add_attribute("version", config.version.to_string())
        .add_attribute("admin", config.admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ReceiveNft(msg) => handle_receive_cw721(deps, env, info, msg),
        ExecuteMsg::RentNft {
            token_id,
            cw721_contract,
        } => rent_nft(deps, env, info, token_id, cw721_contract),
        ExecuteMsg::EditLendingOrder {
            token_id,
            cw721_contract,
            lend_amount,
            lend_time,
        } => edit_lend_order(deps, info, token_id, lend_amount, lend_time, cw721_contract),
        ExecuteMsg::DelistNft {
            token_id,
            cw721_contract,
        } => delist_nft(deps, env, info, token_id, cw721_contract),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractInfo {} => to_binary(&query::config(deps)?),
        QueryMsg::LendOrder { token_id, cw721_contract } => to_binary(&query::query_lend_order(deps, token_id, cw721_contract)?),
        QueryMsg::RentOrder { token_id, cw721_contract } => to_binary(&query::query_rent_order(deps, token_id, cw721_contract)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
