use schemars::JsonSchema;
use serde::{ Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Addr}; 
use secret_toolkit::{ 
    storage:: { Item, Keymap, AppendStore }
}; 

pub static CONFIG_KEY: &[u8] = b"config"; 
pub const ADMIN_KEY: &[u8] = b"admin"; 
pub const INHOLDING_NFT_KEY: &[u8] = b"inholding_nft";
pub const PREFIX_REVOKED_PERMITS: &str = "revoke"; 

pub static CONFIG_ITEM: Item<State> = Item::new(CONFIG_KEY); 
pub static ADDRESS_COLLECTION_STORE: Keymap<String, Addr> = Keymap::new(INHOLDING_NFT_KEY);


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {  
    pub owner: Addr,    
    pub viewing_key: String,
    pub nft_contract_address: Addr,
    pub nft_code_hash: String
} 