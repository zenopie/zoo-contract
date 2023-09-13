use cosmwasm_std::{Response, StdError, StdResult, Uint128, Addr, Env, DepsMut, 

};

use crate::state::{STATE, TICKETS, Ticket, TICKETLOG};



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

    let mut i = 0;
    while i < quantity {
        let ticket = Ticket{
            owner: from.clone(),
        };

        //first ticket is zero
        let ticket_number = state.entries + i;
        ticket_log.push(ticket_number);
        let current_drawing = TICKETS.add_suffix(state.drawing_end.to_string().as_bytes());
        current_drawing.insert(deps.storage, &ticket_number, &ticket).unwrap();

        i += 1;
    }
    ticket_storage.insert(deps.storage, &from, &ticket_log).unwrap();

    state.entries += quantity;
    state.next_jackpot += (amount.u128() * 90) / 100;
    STATE.save(deps.storage, &state).unwrap();

    Ok(Response::default())
}