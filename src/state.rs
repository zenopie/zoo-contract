use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp, Uint128};

use secret_toolkit_storage::{Keymap, Item};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub active_drawing: bool,
    pub entries: u32,
    pub drawing_end: Timestamp,
    pub winner: Addr,
    pub message: String,
    pub max_bet: Uint128,
    pub known_snip: Addr,
    pub snip_hash: String,
    pub jackpot: u128,
    pub next_jackpot: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Admin {
    pub admin: Addr,
    pub vault: u128,

}

pub static STATE: Item<State> = Item::new(b"state");

pub static ADMIN: Item<Admin> = Item::new(b"admin");



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Ticket {
    pub owner: Addr,
}

pub static TICKETS: Keymap<u32, Ticket> = Keymap::new(b"tickets");

pub static TICKETLOG: Keymap<Addr, Vec<u32>> = Keymap::new(b"ticketlog");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Card {
    pub suit: String,
    pub number: u32, 
    pub face: String,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Blackjack {
    pub action: String,
    pub dealer: Vec<Card>, 
    pub player : Vec<Card>,
    pub split: Vec<Card>,
    pub split_result: String,
    pub deck: Vec<Card>,
    pub wager: u128,
}

pub static BLACKJACK: Keymap<Addr, Blackjack> = Keymap::new(b"blackjack");