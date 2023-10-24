use cosmwasm_std::{Addr, WasmMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    ensure, from_binary, to_binary, Binary, DepsMut, Env, MessageInfo, Response, Uint128,
};
use cw721::{Cw721ReceiveMsg, Expiration};
use cw721_base::ExecuteMsg::Mint;
use crate::{error::ContractError, msg::{TokenExtension, MetadataAttribute}, utils::encode_binary};
use serde::Serialize;

use crate::{
    msg::{Cw721HookMsg, LendDetails},
    state::{UserInfo, LENDED_ITEMS},
};

// 1 day in seconds
const ONE_DAY: u64 = 86_400;
// 1 minute in seconds
const ONE_MINUTE: u64 = 60;
// 1 year in seconds
const ONE_YEAR: u64 = 31_536_000;

const DENOM: &str = "test";

const ORIGINAL_TOKEN_ID: &str = "original_token_id";
const ORIGINAL_TOKEN_ADDRESS: &str = "original_token_address";

// fn set_user(
//     deps: DepsMut,
//     info: MessageInfo,
//     user_info: UserInfo,
// ) -> Result<Response, ContractError> {
//     Ok(Response::new())
// }

fn lend_nft(
    deps: DepsMut,
    env: Env,
    nft_id: String,
    lend_time: Expiration,
    lend_amount: Uint128,
    cw721_contract: String,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

fn rent_nft(
    deps: DepsMut,
    info: MessageInfo,
    user_info: UserInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

fn redeem_nft(
    deps: DepsMut,
    info: MessageInfo,
    user_info: UserInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

fn sublet_nfts(
    deps: DepsMut,
    info: MessageInfo,
    user_info: UserInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

fn delist_nft(
    deps: DepsMut,
    info: MessageInfo,
    user_info: UserInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

fn edit_lending_order(
    deps: DepsMut,
    info: MessageInfo,
    user_info: UserInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

fn execute_lend(
    deps: DepsMut,
    env: Env,
    sender: String,
    nft_id: String,
    lend_amount: Uint128,
    lend_time: u64,
    cw721_contract: String,
) -> Result<Response, ContractError> {
    // Lock time can't be too long
    ensure!(lend_time <= ONE_YEAR, ContractError::LendTimeTooLong {});

    // Lock time can't be too short
    ensure!(lend_time >= ONE_DAY, ContractError::LendTimeTooShort {});

    // Concatenate NFT's contract address and ID to form a unique ID for each NFT
    let lend_id = format!("{cw721_contract}{nft_id}");

    // Make sure NFT isn't already locked in this contract
    let lend_id_check = LENDED_ITEMS.may_load(deps.storage, &lend_id)?;
    ensure!(lend_id_check.is_none(), ContractError::LendedNFT {});

    // Add lock time to current block time
    let expiration_time = env.block.time.plus_seconds(lend_time);

    // Set lock details
    let lend_details = LendDetails {
        expiration: Expiration::AtTime(expiration_time),
        nft_id,
        nft_contract: cw721_contract,
        lend_amount
    };

    // Save all the details. The key represents the concatenated lock_id & the value represents the lock details
    LENDED_ITEMS.save(deps.storage, &lend_id, &lend_details)?;

    Ok(Response::new()
        .add_attribute("action", "locked_nft")
        // The recipient should keep the lock ID to easily claim the NFT
        .add_attribute("lock_id", lend_id))
}

// fn execute_claim(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     lock_id: String,
// ) -> Result<Response, ContractError> {
//     // nonpayable(&info)?;
//     // Check if lock ID exists
//     let locked_item = LENDED_ITEMS.may_load(deps.storage, &lock_id)?;

//     if let Some(locked_nft) = locked_item {
//         // Check if lock is expired
//         let expiration = locked_nft.expiration;
//         ensure!(
//             expiration.is_expired(&env.block),
//             ContractError::LendedNFT {}
//         );

//         // Remove NFT from the list of locked items
//         LOCKED_ITEMS.remove(deps.storage, &lock_id);

//         Ok(Response::new()
//             // Send NFT to the recipient
//             .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
//                 contract_addr: locked_nft.nft_contract,
//                 msg: encode_binary(&Cw721ExecuteMsg::TransferNft {
//                     recipient: locked_nft.recipient,
//                     token_id: locked_nft.nft_id,
//                 })?,
//                 funds: vec![],
//             }))
//             .add_attribute("action", "claimed_nft"))
//     } else {
//         Err(ContractError::NFTNotFound {})
//     }
// }

pub fn handle_receive_cw721(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    println!("cw721_msg {:?}", msg);
    // let sender = deps.api.addr_validate(msg.sender.as_str())?;
    match from_binary(&msg.msg)? {
        Cw721HookMsg::LendNft {
            lend_time,
            lend_amount,
        } => lend_nft(
            deps,
            env,
            msg.token_id,
            lend_time,
            lend_amount,
            info.sender.to_string(),
        ),
    }
}

fn execute_wrap(
    deps: DepsMut,
    env: Env,
    sender: String,
    token_id: String,
    token_address: Addr,
    wrapped_token_id: Option<String>,
    cw721_contract: String,
) -> Result<Response, ContractError> {
    ensure!(
        token_address != env.contract.address,
        ContractError::CannotDoubleWrapToken {}
    );

    let wrapped_token_id = wrapped_token_id.unwrap_or_else(|| token_id.to_string());
    let extension = TokenExtension {
        name: wrapped_token_id.clone(),
        publisher: sender.clone(),
        description: None,
        attributes: vec![
            MetadataAttribute {
                trait_type: ORIGINAL_TOKEN_ID.to_owned(),
                value: token_id.clone(),
                display_type: None,
            },
            MetadataAttribute {
                trait_type: ORIGINAL_TOKEN_ADDRESS.to_owned(),
                value: token_address.to_string(),
                display_type: None,
            },
        ],
        image: String::from(""),
        image_data: None,
        external_url: None,
        animation_url: None,
        youtube_url: None,
    };
    let mint_msg = Mint {
        token_id: wrapped_token_id.to_string(),
        owner: sender,
        token_uri: None,
        extension,
    };
    let msg = encode_binary(&mint_msg)?;
    let cw721_contract_addr = ANDROMEDA_CW721_ADDR.load(deps.storage)?;
    let wasm_msg = WasmMsg::Execute {
        contract_addr: cw721_contract_addr.clone(),
        funds: vec![],
        msg,
    };
    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "wrap")
        .add_attribute("token_id", token_id)
        .add_attribute("token_address", token_address)
        .add_attribute("wrapped_token_id", wrapped_token_id)
        .add_attribute("wrapped_token_address", cw721_contract_addr))
}

// fn execute_unwrap(
//     deps: DepsMut,
//     sender: String,
//     token_id: String,
//     token_address: Addr,
// ) -> Result<Response, ContractError> {
//     let can_unwrap = CAN_UNWRAP.load(deps.storage)?;
//     let cw721_contract_addr = ANDROMEDA_CW721_ADDR.load(deps.storage)?;
//     ensure!(can_unwrap, ContractError::UnwrappingDisabled {});
//     ensure!(
//         token_address == cw721_contract_addr,
//         ContractError::TokenNotWrappedByThisContract {}
//     );
//     let (original_token_id, original_token_address) =
//         get_original_nft_data(&deps.querier, token_id.clone(), token_address.to_string())?;

//     let burn_msg = Cw721ExecuteMsg::Burn { token_id };
//     let transfer_msg = Cw721ExecuteMsg::TransferNft {
//         recipient: sender,
//         token_id: original_token_id,
//     };
//     Ok(Response::new()
//         .add_message(WasmMsg::Execute {
//             contract_addr: cw721_contract_addr,
//             funds: vec![],
//             msg: encode_binary(&burn_msg)?,
//         })
//         .add_message(WasmMsg::Execute {
//             contract_addr: original_token_address,
//             funds: vec![],
//             msg: encode_binary(&transfer_msg)?,
//         })
//         .add_attribute("action", "unwrap"))
// }
