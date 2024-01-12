use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use crate::msg::BetMsg;

use secret_toolkit_storage::{Keymap, Item};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub max_bet: Uint128,
    pub known_snip: Addr,
    pub snip_hash: String,
    pub decimal: Uint128,
    pub vault: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Admin {
    pub admin: Addr,

}

pub static STATE: Item<State> = Item::new(b"state");

pub static ADMIN: Item<Admin> = Item::new(b"admin");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LastSpin {
    pub winning_number: u32,
    pub bets: Box<[BetMsg]>,
    pub winnings: Uint128,
}

pub static LASTSPIN: Keymap<Addr, LastSpin> = Keymap::new(b"lastspin");


