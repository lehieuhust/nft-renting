use crate::{
    error::ContractError,
    msg::RentDetails,
    state::RENTED_ITEMS,
    utils::encode_binary,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    ensure, from_binary, DepsMut, Env, MessageInfo, Response, Uint128,
};
use cosmwasm_std::{WasmMsg, CosmosMsg, BankMsg, coins};
use cw721::{Cw721ReceiveMsg, Expiration, Cw721ExecuteMsg};


use crate::{
    msg::{Cw721HookMsg, LendDetails},
    state::LENDED_ITEMS,
};

const DENOM: &str = "ucmdx";
// 1 minute in seconds
const ONE_MINUTE: u64 = 60;
// 1 year in seconds
const ONE_YEAR: u64 = 31_536_000;

pub fn handle_receive_cw721(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&msg.msg)? {
        Cw721HookMsg::LendNft {
            lend_time,
            lend_amount,
        } => create_lend_order(
            deps,
            msg.token_id,
            msg.sender,
            lend_amount,
            lend_time,
            info.sender.to_string(),
        ),
    }
}

fn create_lend_order(
    deps: DepsMut,
    nft_id: String,
    sender: String,
    lend_amount: Uint128,
    lend_time: u64,
    cw721_contract: String,
) -> Result<Response, ContractError> {
    // Lock time can't be too long
    ensure!(lend_time <= ONE_YEAR, ContractError::LendTimeTooLong {});

    // Lock time can't be too short
    ensure!(lend_time >= ONE_MINUTE, ContractError::LendTimeTooShort {});

    // Concatenate NFT's contract address and ID to form a unique ID for each NFT
    let lend_id = format!("{cw721_contract}{nft_id}");

    // Make sure NFT isn't already locked in this contract
    let lend_id_check = LENDED_ITEMS.may_load(deps.storage, &lend_id)?;
    ensure!(lend_id_check.is_none(), ContractError::LendedNFT {});

    // Set lock details
    let lend_details = LendDetails {
        lender: sender,
        lend_time,
        nft_id,
        nft_contract: cw721_contract,
        lend_amount,
    };

    // Save all the details. The key represents the concatenated lock_id & the value represents the lock details
    LENDED_ITEMS.save(deps.storage, &lend_id, &lend_details)?;

    Ok(Response::new()
        .add_attribute("action", "create_lend_order")
        .add_attribute("lend_id", lend_id)
        .add_attribute("lender", lend_details.lender.clone())
        .add_attribute("token_id", lend_details.nft_id.clone())
        .add_attribute("lendtime", format!("{:?}", lend_details.lend_time.clone()))
    )
}

pub fn rent_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    nft_id: String,
    cw721_contract: String,
) -> Result<Response, ContractError> {
    // Concatenate NFT's contract address and ID to form a unique ID for each NFT
    let lend_id = format!("{cw721_contract}{nft_id}");

    // Check if lend ID exists
    let lended_item = LENDED_ITEMS.load(deps.storage, &lend_id)?;

    if lended_item.lender == info.sender.as_str() {
        return Err(ContractError::RenterCannotBeLender {});
    }

    // Add lock time to current block time
    let expiration_time = env.block.time.plus_seconds(lended_item.lend_time);

    // Set rent details
    let rent_details = RentDetails {
        renter: info.sender.as_str().to_owned(),
        expiration: Expiration::AtTime(expiration_time),
        nft_id,
        nft_contract: cw721_contract,
        rent_amount: lended_item.lend_amount,
    };

    RENTED_ITEMS.save(deps.storage, &lend_id, &rent_details)?;
    let mut msgs: Vec<CosmosMsg> = vec![];
    msgs.push(CosmosMsg::Bank(BankMsg::Send {
        to_address: lended_item.lender,
        amount: coins(Uint128::from(rent_details.rent_amount).into(), DENOM),
    }));

    Ok(Response::new()
        .add_attribute("action", "rent_nft")
        .add_attribute("renter", rent_details.renter.clone())
        .add_attribute("token_id", rent_details.nft_id.clone())
        .add_attribute("expiration", format!("{:?}", rent_details.expiration.clone()))
        // Send NFT to the recipient
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: lended_item.nft_contract,
            msg: encode_binary(&Cw721ExecuteMsg::Approve {
                spender: rent_details.renter,
                token_id: rent_details.nft_id,
                expires: Some(rent_details.expiration),
            })?,
            funds: vec![],
        }))
        .add_messages(msgs)
    )
}

pub fn delist_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    nft_id: String,
    cw721_contract: String,
) -> Result<Response, ContractError> {
    // Concatenate NFT's contract address and ID to form a unique ID for each NFT
    let lend_id = format!("{cw721_contract}{nft_id}");

    // Check if lend ID exists
    let lended_item = LENDED_ITEMS.load(deps.storage, &lend_id)?;
    let rented_item = RENTED_ITEMS.may_load(deps.storage, &lend_id)?;
    
    if lended_item.lender != info.sender.as_str() {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(rented_item) = rented_item {
        // Check if lock is expired
        let expiration = rented_item.expiration;
        ensure!(
            expiration.is_expired(&env.block),
            ContractError::RentedNFT {}
        );
    }

    LENDED_ITEMS.remove(deps.storage, &lend_id);

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: lended_item.nft_contract,
            msg: encode_binary(&Cw721ExecuteMsg::TransferNft {
                recipient: lended_item.lender,
                token_id: lended_item.nft_id,
            })?,
            funds: vec![],
        }))
        .add_attribute("action", "delist_nft")
        .add_attribute("lend_id", lend_id)
    )
}

pub fn edit_lend_order(
    deps: DepsMut,
    info: MessageInfo,
    nft_id: String,
    lend_amount: Uint128,
    lend_time: u64,
    cw721_contract: String,
) -> Result<Response, ContractError> {
    // Concatenate NFT's contract address and ID to form a unique ID for each NFT
    let lend_id = format!("{cw721_contract}{nft_id}");

    // Make sure NFT isn't already locked in this contract
    let mut lended_items = LENDED_ITEMS.load(deps.storage, &lend_id)?;

    if lended_items.lender != info.sender.as_str() {
        return Err(ContractError::Unauthorized {});
    }

    // Lock time can't be too long
    ensure!(lend_time <= ONE_YEAR, ContractError::LendTimeTooLong {});

    // Lock time can't be too short
    ensure!(lend_time >= ONE_MINUTE, ContractError::LendTimeTooShort {});
    
    // Set lock details
    lended_items = LendDetails {
        lender: info.sender.as_str().to_owned(),
        lend_time,
        nft_id,
        nft_contract: cw721_contract,
        lend_amount,
    };

    // Save all the details. The key represents the concatenated lock_id & the value represents the lock details
    LENDED_ITEMS.save(deps.storage, &lend_id, &lended_items)?;

    Ok(Response::new()
        .add_attribute("action", "edit_lend_order")
        .add_attribute("lend_id", lend_id)
        .add_attribute("lender", lended_items.lender.clone())
        .add_attribute("token_id", lended_items.nft_id.clone())
        .add_attribute("lendtime", format!("{:?}", lended_items.lend_time.clone()))
    )
}
