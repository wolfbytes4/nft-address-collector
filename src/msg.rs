use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{
   Addr, Binary
};
use secret_toolkit::{ 
    permit:: { Permit },
    snip721::{
        ViewerInfo
    }
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {  
    pub entropy: String,
    pub nft_contract_address: Addr,
    pub nft_code_hash: String
}  

 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AddressCollect {
    pub token_id: String,
    pub address: Addr
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AddressResponse {
    pub addresses: Vec<AddressCollect>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg { 
    RevokePermit{
        permit_name: String
    },  
    CollectAddress{ 
        token_ids: Vec<String>,
        wallet_address: Addr
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {   
    GetAddresses {
        viewer: ViewerInfo, 
        start_page: u32, 
        page_size: u32
    }
}