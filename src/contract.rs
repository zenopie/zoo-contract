use cosmwasm_std::{
    entry_point, to_binary, from_binary, Binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Addr, Uint128, CosmosMsg,
    WasmMsg,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse, Snip20Msg,
    ReceiveMsg, LastRouletteResponse
};
use crate::state::{STATE, State, ADMIN, Admin, LASTSPIN, LastSpin};
use crate::roulette::{roulette_receive};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let state = State {
        known_snip: msg.known_snip,
        snip_hash: msg.snip_hash,
        decimal: msg.decimal,
        max_bet: msg.max_bet,
        vault: Uint128::zero(),
    };
    let admin = Admin {
        admin: info.sender.clone(),
    };

    STATE.save(deps.storage, &state)?;
    ADMIN.save(deps.storage, &admin)?;

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
        ExecuteMsg::ChangeMaxBet {max} => change_max_bet(deps, env, info, max),
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

    let mut state = STATE.load(deps.storage).unwrap();
    state.vault -= amount;
    STATE.save(deps.storage, &state).unwrap();
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

pub fn change_max_bet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    max: Uint128,
) -> StdResult<Response> {
    
    let admin = ADMIN.load(deps.storage).unwrap();
    if info.sender != admin.admin {
        return Err(StdError::generic_err("not authorized"));
    }
    let mut state = STATE.load(deps.storage).unwrap();
    state.max_bet = max;
    STATE.save(deps.storage, &state).unwrap();
    Ok(Response::default())
}


pub fn try_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _sender: Addr,
    from: Addr,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, StdError> {

    let msg: ReceiveMsg = from_binary(&msg)?;

    let state = STATE.load(deps.storage)?;
    if info.sender != state.known_snip {
        return Err(StdError::generic_err("invalid snip"));
    }

    match msg {
        ReceiveMsg::Roulette{bets} => roulette_receive(deps, env, from, amount, bets),
        ReceiveMsg::Deposit {} => deposit_receive(deps, env, from, amount),
    }   
}

pub fn deposit_receive(
    deps: DepsMut,
    _env: Env,
    from: Addr,
    amount: Uint128,
) -> StdResult<Response> {

    let admin = ADMIN.load(deps.storage)?;
    if from != admin.admin {
        return Err(StdError::generic_err("not authorized"));
    }
    let mut state = STATE.load(deps.storage)?;
    state.vault += amount;
    STATE.save(deps.storage, &state)?;
    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&query_state(deps)?),
        QueryMsg::LastRoulette {address} => to_binary(&query_last_roulette(deps, address)?),
    }
}

fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(StateResponse { state: state })
}


fn query_last_roulette(deps: Deps, address: Addr) -> StdResult<LastRouletteResponse> {

    let mut last_roulette_option:Option<LastSpin> = LASTSPIN.get(deps.storage, &address);
    if last_roulette_option.is_none() {
        last_roulette_option = Some(LastSpin {
            winning_number: 404,
            bets: Vec::new().into(),
            winnings: Uint128::from(404u32),
        });
    }
    let last_roulette:LastSpin = last_roulette_option.unwrap();

    Ok(LastRouletteResponse {
        last_spin: last_roulette,
    })
}