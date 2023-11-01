use cosmwasm_std::{
    entry_point, to_binary, from_binary, Binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Addr, Uint128, CosmosMsg,
    WasmMsg,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse, Snip20Msg,
    ReceiveMsg, BjStateResponse, TicketLogResponse, LastRaffleResponse, LastRouletteResponse
};
use crate::state::{STATE, State, ADMIN, Admin, Blackjack, BLACKJACK, TICKETLOG, LASTSPIN, LastSpin};
use crate::operations::{deposit_receive};
use crate::blackjack::{try_blackjack, blackjack_receive};
use crate::roulette::{roulette_receive};
use crate::raffle::{raffle_receive, try_raffle};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let state = State {
        drawing_end: env.block.time.clone().plus_seconds(604800),
        last_drawing: env.block.time.clone(),
        entries: 0,
        known_snip: msg.known_snip,
        snip_hash: msg.snip_hash,
        winner: 0,
        max_bet: msg.max_bet,
        jackpot: 0,
        next_jackpot: 0,
    };
    let admin = Admin {
        admin: info.sender.clone(),
    };

    STATE.save(deps.storage, &state).unwrap();
    ADMIN.save(deps.storage, &admin).unwrap();

    let msg = to_binary(&Snip20Msg::register_receive(env.contract.code_hash))?;
    let message = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: state.known_snip.into_string(),
        code_hash: state.snip_hash,
        msg,
        funds: vec![],
    });
    Ok(Response::new().add_message(message))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::Withdraw {amount} => try_withdraw(deps, env, info, amount),
        ExecuteMsg::ChangeAdmin {address} => change_admin(deps, env, info, address),
        ExecuteMsg::Raffle {} => try_raffle(deps, env, info),
        ExecuteMsg::Blackjack {action} => try_blackjack(deps, env, info, action),
        ExecuteMsg::Receive {
            sender,
            from,
            amount,
            msg,
            memo: _,
        } => try_receive(deps, env, info, sender, from, amount, msg),  
    }
}

pub fn try_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    
    let admin = ADMIN.load(deps.storage).unwrap();
    if info.sender != admin.admin {
        return Err(StdError::generic_err("not authorized"));
    }

    let state = STATE.load(deps.storage).unwrap();
    let msg = to_binary(&Snip20Msg::transfer_snip(
        info.sender,
        amount,
    ))?;
    let message = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: state.known_snip.to_string(),
        code_hash: state.snip_hash,
        msg,
        funds: vec![],
    });
    let response = Response::new()
    .add_message(message);
    Ok(response)
}

pub fn change_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: Addr,
) -> StdResult<Response> {
    
    let admin = ADMIN.load(deps.storage).unwrap();
    if info.sender != admin.admin {
        return Err(StdError::generic_err("not authorized"));
    }
    let new_admin = Admin {
        admin: address,
    };
    ADMIN.save(deps.storage, &new_admin).unwrap();
    Ok(Response::default())
}



pub fn try_receive(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _sender: Addr,
    from: Addr,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, StdError> {

    let msg: ReceiveMsg = from_binary(&msg)?;

    match msg {
        ReceiveMsg::Roulette{bets} => roulette_receive(deps, env, from, amount, bets),
        ReceiveMsg::Raffle{quantity} => raffle_receive(deps, env, from, amount, quantity),
        ReceiveMsg::Blackjack{action} => blackjack_receive(deps, env, from, amount, action),
        ReceiveMsg::Deposit {} => deposit_receive(deps, env, from, amount),
    }   
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&query_state(deps)?),
        QueryMsg::BjState {address} => to_binary(&query_bjstate(deps, address)?),
        QueryMsg::TicketLog {address} => to_binary(&query_ticket_log(deps, address)?),
        QueryMsg::LastRaffle {address} => to_binary(&query_last_raffle(deps, address)?),
        QueryMsg::LastRoulette {address} => to_binary(&query_last_roulette(deps, address)?),
    }
}

fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage).unwrap();
    Ok(StateResponse { state: state })
}

fn query_bjstate(deps: Deps, address: Addr) -> StdResult<BjStateResponse> {

    let mut bj_state_option:Option<Blackjack> = BLACKJACK.get(deps.storage, &address);
    if bj_state_option.is_none() {
        bj_state_option = Some(Blackjack{
            action: "ready".to_string(),
            dealer: Vec::new(), 
            player : Vec::new(),
            split: Vec::new(),
            split_result: "nosplit".to_string(),
            deck: Vec::new(),
            wager: 0,
            result: "no history".to_string(),
            winnings: 0,
        })
    } 
    let mut bj_state:Blackjack = bj_state_option.unwrap();
    if bj_state.result == "in progress".to_string() {
        bj_state.dealer.remove(0);
    }
    Ok(BjStateResponse { state: bj_state })
}

fn query_ticket_log(deps: Deps, address: Addr) -> StdResult<TicketLogResponse> {

    let state = STATE.load(deps.storage).unwrap();
    let ticket_storage = TICKETLOG.add_suffix(state.drawing_end.to_string().as_bytes());
    let mut ticket_log_option:Option<Vec<u32>> = ticket_storage.get(deps.storage, &address);
    if ticket_log_option.is_none() {
        ticket_log_option = Some(Vec::new());
    }
    let ticket_log:Vec<u32> = ticket_log_option.unwrap();

    Ok(TicketLogResponse { tickets: ticket_log })
}

fn query_last_raffle(deps: Deps, address: Addr) -> StdResult<LastRaffleResponse> {

    let state = STATE.load(deps.storage).unwrap();
    let ticket_storage = TICKETLOG.add_suffix(state.last_drawing.to_string().as_bytes());
    let mut ticket_log_option:Option<Vec<u32>> = ticket_storage.get(deps.storage, &address);
    if ticket_log_option.is_none() {
        ticket_log_option = Some(Vec::new());
    }
    let ticket_log:Vec<u32> = ticket_log_option.unwrap();

    Ok(LastRaffleResponse {
        winner: state.winner,
        tickets: ticket_log,
    })
}

fn query_last_roulette(deps: Deps, address: Addr) -> StdResult<LastRouletteResponse> {

    let mut last_roulette_option:Option<LastSpin> = LASTSPIN.get(deps.storage, &address);
    if last_roulette_option.is_none() {
        last_roulette_option = Some(LastSpin {
            winning_number: 404,
            bets: Vec::new().into(),
            winnings: 404,
        });
    }
    let last_roulette:LastSpin = last_roulette_option.unwrap();

    Ok(LastRouletteResponse {
        last_spin: last_roulette,
    })
}