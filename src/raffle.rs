use cosmwasm_std::{Response, StdError, StdResult, Uint128, Addr, Env, DepsMut, CosmosMsg, to_binary, MessageInfo,
    WasmMsg,

};

use crate::state::{STATE, TICKETS, Ticket, TICKETLOG, ADMIN};
use crate::msg::{Snip20Msg,};



pub fn try_raffle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> StdResult<Response> {

    let mut state = STATE.load(deps.storage).unwrap();

    let admin = ADMIN.load(deps.storage).unwrap();
    if info.sender != admin.admin {
        if state.drawing_end < env.block.time {
            return Err(StdError::generic_err("must wait until drawing end"));
        }
    }

    let random_binary = env.block.random.clone();
    let random_bytes = &random_binary.as_ref().unwrap().0;
        
    let random_number = u32::from_le_bytes([
        random_bytes[0],
        random_bytes[1],
        random_bytes[2],
        random_bytes[3],
    ]);
    let spin = random_number % state.entries;

    let current_drawing = TICKETS.add_suffix(state.drawing_end.to_string().as_bytes());
    let winning_ticket = current_drawing.get(deps.storage, &spin).unwrap();
    let winnings = Uint128::from(state.jackpot);
 
    let msg = to_binary(&Snip20Msg::transfer_snip(
        winning_ticket.owner.clone(),
        winnings,
    ))?;

    state.winner = spin;
    state.last_drawing = state.drawing_end.clone();
    state.drawing_end = env.block.time.plus_seconds(604800);
    state.jackpot = state.next_jackpot;
    state.next_jackpot = 0;
    state.entries = 0;

    STATE.save(deps.storage, &state).unwrap();

    let message = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: state.known_snip.to_string(),
        code_hash: state.snip_hash,
        msg,
        funds: vec![],
    });
    let response = Response::new()
    .add_message(message)
    .add_attribute("winning_ticket", spin.to_string())
    .add_attribute("winnings", winnings.to_string())
    .add_attribute("test", winning_ticket.owner.to_string());
    Ok(response)
}

pub fn raffle_receive(
    deps: DepsMut,
    _env: Env,
    from: Addr,
    amount: Uint128,
    quantity: u32,
) -> StdResult<Response> {

    if amount.u128() != (quantity * 1000000).into() {
        return Err(StdError::generic_err("invalid amount"));
    }

    let mut state = STATE.load(deps.storage).unwrap();

    let ticket_storage = TICKETLOG.add_suffix(state.drawing_end.to_string().as_bytes());
    let mut ticket_log_option:Option<Vec<u32>> = ticket_storage.get(deps.storage, &from);
    if ticket_log_option.is_none() {
        ticket_log_option = Some(Vec::new());
    }
    let mut ticket_log:Vec<u32> = ticket_log_option.unwrap();
    let mut new_tickets:Vec<u32> = Vec::new();
    let current_drawing = TICKETS.add_suffix(state.drawing_end.to_string().as_bytes());
    let mut i = 0;
    while i < quantity {
        let ticket = Ticket{
            owner: from.clone(),
        };

        //first ticket is zero
        let ticket_number = state.entries + i;
        ticket_log.push(ticket_number);
        new_tickets.push(ticket_number);
        current_drawing.insert(deps.storage, &ticket_number, &ticket).unwrap();

        i += 1;
    }
    ticket_storage.insert(deps.storage, &from, &ticket_log).unwrap();
    state.entries += quantity;
    state.next_jackpot += (amount.u128() * 90) / 100;
    STATE.save(deps.storage, &state).unwrap();

    let new_tickets_string = new_tickets
    .into_iter()
    .map(|c| c.to_string())
    .collect::<Vec<String>>()
    .join(",");

    let response = Response::new()
    .add_attribute("tickets", new_tickets_string);
    Ok(response)
}