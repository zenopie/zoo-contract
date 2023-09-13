use cosmwasm_std::{
    entry_point, to_binary, from_binary, Binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Addr, Uint128, CosmosMsg,
    WasmMsg,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse, Snip20Msg,
    ReceiveMsg, BjStateResponse,
};
use crate::state::{STATE, State, ADMIN, Admin, TICKETS, Blackjack, BLACKJACK};
use crate::operations::{deposit_receive};
use crate::blackjack::{try_blackjack, blackjack_receive};
use crate::roulette::{roulette_receive};
use crate::raffle::{raffle_receive};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let state = State {
        active_drawing: false,
        drawing_end: env.block.time.clone().plus_seconds(604800),
        entries: 0,
        known_snip: msg.known_snip,
        snip_hash: msg.snip_hash,
        winner: info.sender.clone(),
        message: "🌎".to_string(),
        max_bet: msg.max_bet,
        jackpot: 0,
        next_jackpot: 0,
    };
    let admin = Admin {
        admin: info.sender.clone(),
        vault: 0,
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
        ExecuteMsg::Test {amount} => try_test(deps, env, info, amount),
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

pub fn try_test(
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

pub fn try_raffle(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> StdResult<Response> {

    let mut state = STATE.load(deps.storage).unwrap();


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
        })
    }
    let mut bj_state:Blackjack = bj_state_option.unwrap();
    bj_state.dealer.remove(0);

    Ok(BjStateResponse { state: bj_state })
}
