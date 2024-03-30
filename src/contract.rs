use cosmwasm_std::{
    entry_point, to_binary, Env, Deps, DepsMut,
    MessageInfo, Response, StdError, StdResult, Addr, CanonicalAddr,
    Binary, CosmosMsg
};
use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, AddressCollect, AddressResponse };
use crate::state::{ State, CONFIG_ITEM, ADDRESS_COLLECTION_STORE, PREFIX_REVOKED_PERMITS};
use crate::rand::{sha_256};
use secret_toolkit::{
    snip721::{
        set_viewing_key_msg, owner_of_query, OwnerOf, ViewerInfo
    },
    permit::{validate, Permit, RevokedPermits}
};  
pub const BLOCK_SIZE: usize = 256;


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg
) -> Result<Response, StdError> {
    let prng_seed: Vec<u8> = sha_256(base64::encode(msg.entropy).as_bytes()).to_vec();
    let viewing_key = base64::encode(&prng_seed);

    // create initial state
    let state = State { 
        viewing_key: viewing_key,
        owner: info.sender.clone(),
        nft_contract_address: Addr::unchecked(msg.nft_contract_address.to_string()),
        nft_code_hash: msg.nft_code_hash.to_string()
    };

    //Save Contract state
    CONFIG_ITEM.save(deps.storage, &state)?;
 
    deps.api.debug(&format!("Contract was initialized by {}", info.sender));
      
    Ok(Response::new()
        .add_message(set_viewing_key_msg(
            state.viewing_key.to_string(),
            None,
            BLOCK_SIZE,
            msg.nft_code_hash.to_string(),
            msg.nft_contract_address.to_string()
    )?)) 
    
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> Result<Response, StdError> {
    match msg { 
        ExecuteMsg::RevokePermit { permit_name } => {
            try_revoke_permit(deps, &info.sender, &permit_name)
        },
        ExecuteMsg::CollectAddress { token_ids, wallet_address } => {
            try_collect_address(deps, _env, &info.sender, token_ids, wallet_address)
        }
    }
} 

fn try_collect_address(
    deps: DepsMut,
    _env: Env,
    sender: &Addr,
    token_ids: Vec<String>,
    wallet_address: Addr,
) -> Result<Response, StdError> {  
    let state = CONFIG_ITEM.load(deps.storage)?;  
    let vk: ViewerInfo = {
        ViewerInfo {
            address: _env.contract.address.to_string(),
            viewing_key: state.viewing_key,
        }
    };
    for id in token_ids.iter() { 
        let owner: OwnerOf = owner_of_query(
                deps.querier,
                id.to_string(),
                Some(vk.clone()),
                None,
                BLOCK_SIZE,
                state.nft_code_hash.clone(),
                state.nft_contract_address.to_string(),
        )?;

        if owner.owner.unwrap() == sender.to_string() {
            ADDRESS_COLLECTION_STORE.insert(deps.storage, &id, &wallet_address)?;
        }
        else{
            return Err(StdError::generic_err(
            "You do not own this token"
            ));
        }
    } 
    Ok(Response::default())  
}

 
fn try_revoke_permit(
    deps: DepsMut,
    sender: &Addr,
    permit_name: &str,
) -> Result<Response, StdError> {
    RevokedPermits::revoke_permit(deps.storage, PREFIX_REVOKED_PERMITS, &sender.to_string(), permit_name);
    
    Ok(Response::default())
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {   
             QueryMsg::GetAddresses { viewer, start_page, page_size } => to_binary(&query_addresses(deps, viewer, start_page, page_size)?),
    }
}
 

 
fn query_addresses(
    deps: Deps, viewer: ViewerInfo,
    start_page: u32, 
    page_size: u32
) -> StdResult<AddressResponse> { 
    check_admin_key(deps, viewer)?;
    // let history_store = ADDRESS_COLLECTION_STORE.iter(deps.storage);
    let mut addresses_collected: Vec<AddressCollect> = Vec::new(); 
     
    let collection = ADDRESS_COLLECTION_STORE.paging(deps.storage, start_page, page_size)?;

    for (token_id, address) in collection.iter() {
        addresses_collected.push(AddressCollect {
            token_id: token_id.clone(),
            address: address.clone(),
        });
    }  

    Ok(AddressResponse { addresses: addresses_collected })
}  

fn check_admin_key(deps: Deps, viewer: ViewerInfo) -> StdResult<()> { 
    let state = CONFIG_ITEM.load(deps.storage)?;  
    let prng_seed: Vec<u8> = sha_256(base64::encode(viewer.viewing_key).as_bytes()).to_vec();
    let vk = base64::encode(&prng_seed);

    if vk != state.viewing_key || viewer.address != state.owner {
        return Err(StdError::generic_err(
            "Wrong viewing key for this address or viewing key not set",
        ));
    }

    return Ok(());
}
