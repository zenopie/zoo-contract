use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Binary, Uint128};

use crate::state::{State, LastSpin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub known_snip: Addr,
    pub snip_hash: String,
    pub max_bet: Uint128,
    pub decimal: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Withdraw {
        amount: Uint128,
    },
    Receive {
        sender: Addr,
        from: Addr,
        amount: Uint128,
        memo: Option<String>,
        msg: Binary,
    },
    ChangeAdmin {
        address: Addr,
    },
    ChangeMaxBet {
        max: Uint128,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BetMsg {
    pub bet_type: String,
    pub numbers: String,
    pub wager: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    Roulette {
        bets: Box<[BetMsg]>,
    },
    Deposit {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetState {},
    LastRoulette {address: Addr,},
}
// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct StateResponse {
    pub state: State,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LastRouletteResponse {
    pub last_spin: LastSpin,
}

// Messages sent to SNIP-20 contracts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Snip20Msg {
    RegisterReceive {
        code_hash: String,
        padding: Option<String>,
    },
    Transfer {
        recipient: Addr,
        amount: Uint128,
        padding: Option<String>,
    },
}

impl Snip20Msg {
    pub fn register_receive(code_hash: String) -> Self {
        Snip20Msg::RegisterReceive {
            code_hash,
            padding: None, // TODO add padding calculation
        }
    }
    pub fn transfer_snip(recipient: Addr, amount: Uint128) -> Self {
        Snip20Msg::Transfer {
            recipient,
            amount,
            padding: None, // TODO add padding calculation
        }
    }
}
