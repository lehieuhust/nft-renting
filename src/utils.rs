use cosmwasm_std::{QuerierWrapper, QueryRequest, WasmQuery, Binary, to_binary};
use cw721::{OwnerOfResponse, Cw721QueryMsg};
use serde::{de::DeserializeOwned, Serialize};

use crate::ContractError;

pub fn encode_binary<T>(val: &T) -> Result<Binary, ContractError>
where
    T: Serialize,
{
    match to_binary(val) {
        Ok(encoded_val) => Ok(encoded_val),
        Err(err) => Err(err.into()),
    }
}

pub fn query_cw721<T, M>(
    querier: QuerierWrapper,
    msg: &M,
    cw721_contract: String,
) -> Result<T, ContractError>
where
    T: DeserializeOwned,
    M: Serialize,
{
    let result: T = querier.query_wasm_smart(cw721_contract, &msg)?;
    Ok(result)
}

fn get_token_owner(
    querier: &QuerierWrapper,
    token_id: String,
    cw721_contract: String,
) -> Result<String, ContractError> {
    let res: OwnerOfResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: cw721_contract,
        msg: encode_binary(&Cw721QueryMsg::OwnerOf {
            token_id,
            include_expired: None,
        })?,
    }))?;
    Ok(res.owner)
}
