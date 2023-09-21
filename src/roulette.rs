use cosmwasm_std::{CosmosMsg, Response, to_binary, StdError, StdResult, Uint128, Addr, Env, DepsMut, 
WasmMsg,
};

use crate::msg::{Snip20Msg, BetMsg};
use crate::state::{STATE, LastSpin, LASTSPIN};




pub fn roulette_receive(
    deps: DepsMut,
    env: Env,
    from: Addr,
    amount: Uint128,
    bets: Box<[BetMsg]>,
) -> StdResult<Response> {

    let mut state = STATE.load(deps.storage).unwrap();

    if amount > state.max_bet {
        return Err(StdError::generic_err("above max bet"));
    }


    let mut totalamount = 0u128;
    let mut winnings = 0u128;
    let mut bettotal = 0u128;

    let random_binary = env.block.random.clone();
    let random_bytes = &random_binary.as_ref().unwrap().0;

    let random_number = u32::from_le_bytes([
        random_bytes[0],
        random_bytes[1],
        random_bytes[2],
        random_bytes[3],
    ]);
    let spin = random_number % 36;

    let mut index = 0;
    while index < bets.len() { 

        totalamount += bets[index].wager * 1000000;


        match bets[index].bet_type.as_str() {
            "double_street" => {
                let valid = [
                    "1, 2, 3, 4, 5, 6", "4, 5, 6, 7, 8, 9", "7, 8, 9, 10, 11, 12", "10, 11, 12, 13, 14, 15",
                    "13, 14, 15, 16, 17, 18", "16, 17, 18, 19, 20, 21", "19, 20, 21, 22, 23, 24", "22, 23, 24, 25, 26, 27",
                    "25, 26, 27, 28, 29, 30", "28, 29, 30, 31, 32, 33", "31, 32, 33, 34, 35, 36"
                ];
                let mut i = 0;
                while i < valid.len() {
                    
                    if bets[index].numbers == valid[i] {
                        let valid_split: Vec<&str> = valid[i].split(',').collect();
                        let mut j = 0;
                        while j < valid_split.len() {
                            if spin == valid_split[j].trim().parse::<u32>().unwrap() {
                                winnings += &bets[index].wager * 5;
                                bettotal += &bets[index].wager;
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }

            }
            "street" => {
                let valid = [
                    "1, 2, 3", "4, 5, 6", "7, 8, 9", "10, 11, 12", "13, 14, 15", "16, 17, 18", 
                    "19, 20, 21", "22, 23, 24", "25, 26, 27", "28, 29, 30", "31, 32, 33", "34, 35, 36"
                ];
                let mut i = 0;
                while i < valid.len() {
                    
                    if bets[index].numbers == valid[i] {
                        let valid_split: Vec<&str> = valid[i].split(',').collect();
                        let mut j = 0;
                        while j < valid_split.len() {
                            if spin == valid_split[j].trim().parse::<u32>().unwrap() {
                                winnings += &bets[index].wager * 11;
                                bettotal += &bets[index].wager;
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }
                

            }
            "split" => {
                let valid = [
                    "3, 6", "6, 9", "9, 12", "12, 15", "15, 18", "18, 21", "21, 24", "24, 27",
                    "27, 30", "30, 33", "33, 36", "35, 36", "32, 33", "29, 30", "26, 27", "23, 24",
                    "20, 21", "17, 18", "14, 15", "11, 12", "8, 9", "5, 6", "2, 3", "2, 5", "5, 8",
                    "8, 11", "11, 14", "14, 17", "17, 20", "20, 23", "23, 26", "26, 29", "29, 32",
                    "32, 35", "34, 35", "31, 32", "28, 29", "25, 26", "22, 23", "19, 20", "16, 17",
                    "10, 11", "7, 8", "4, 5", "1, 2", "1, 4", "4, 7", "7, 10", "10, 13", "13, 16",
                    "16, 19", "19, 22", "22, 25", "25, 28", "28, 31", "31, 34", "13, 14",
                ];
                let mut i = 0;
                while i < valid.len() {
                    
                    if bets[index].numbers == valid[i] {
                        let valid_split: Vec<&str> = valid[i].split(',').collect();
                        let mut j = 0;
                        while j < valid_split.len() {
                            if spin == valid_split[j].trim().parse::<u32>().unwrap() {
                                winnings += &bets[index].wager * 17;
                                bettotal += &bets[index].wager;
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }
                
            }
            "corner_bet" => {
                let valid = [
                    "2, 3, 5, 6", "5, 6, 8, 9", "8, 9, 11, 12", "11, 12, 14, 15", "14, 15, 17, 18", "17, 18, 20, 21",
                    "20, 21, 23, 24", "23, 24, 26, 27", "26, 27, 29, 30", "29, 30, 32, 33", "32, 33, 35, 36",
                    "31, 32, 34, 35", "28, 29, 31, 32", "25, 26, 28, 29", "22, 23, 25, 26", "19, 20, 22, 23",
                    "16, 17, 19, 20", "13, 14, 16, 17", "10, 11, 13, 14", "7, 8, 10, 11", "4, 5, 7, 8", "1, 2, 4, 5"
                ];
                let mut i = 0;
                while i < valid.len() {
                    
                    if bets[index].numbers == valid[i] {
                        let valid_split: Vec<&str> = valid[i].split(',').collect();
                        let mut j = 0;
                        while j < valid_split.len() {
                            if spin == valid_split[j].trim().parse::<u32>().unwrap() {
                                winnings += &bets[index].wager * 8;
                                bettotal += &bets[index].wager;
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }
                

            }
            "outside_high_low" => {
                let valid = [
                    "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18",
                    "19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36"
                ];
                let mut i = 0;
                while i < valid.len() {
                    
                    if bets[index].numbers == valid[i] {
                        let valid_split: Vec<&str> = valid[i].split(',').collect();
                        let mut j = 0;
                        while j < valid_split.len() {
                            if spin == valid_split[j].trim().parse::<u32>().unwrap() {
                                winnings += &bets[index].wager;
                                bettotal += &bets[index].wager;
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }
                

            }
            "inside_whole" => {
                if &spin.to_string() == &bets[index].numbers {
                    winnings += &bets[index].wager * 35;
                    bettotal += &bets[index].wager;
                }
            }
            "outside_column" => {
                let valid = [
                    "3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36",
                    "2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35",
                    "1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34"
                ];
                let mut i = 0;
                while i < valid.len() {
                    
                    if bets[index].numbers == valid[i] {
                        let valid_split: Vec<&str> = valid[i].split(',').collect();
                        let mut j = 0;
                        while j < valid_split.len() {
                            if spin == valid_split[j].trim().parse::<u32>().unwrap() {
                                winnings += &bets[index].wager * 2;
                                bettotal += &bets[index].wager;
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }

            }
            "outside_dozen" => {
                let valid = [
                    "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12",
                    "13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24",
                    "25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36"
                ];
                let mut i = 0;
                while i < valid.len() {
                    
                    if bets[index].numbers == valid[i] {
                        let valid_split: Vec<&str> = valid[i].split(',').collect();
                        let mut j = 0;
                        while j < valid_split.len() {
                            if spin == valid_split[j].trim().parse::<u32>().unwrap() {
                                winnings += &bets[index].wager * 2;
                                bettotal += &bets[index].wager;
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }
                

            }
            "outside_oerb" => {

                let valid = [
                    "1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36",
                    "2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36",
                    "2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35",
                    "1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35"
                ];
                let mut i = 0;
                while i < valid.len() {
                    
                    if bets[index].numbers == valid[i] {
                        let valid_split: Vec<&str> = valid[i].split(',').collect();
                        let mut j = 0;
                        while j < valid_split.len() {
                            if spin == valid_split[j].trim().parse::<u32>().unwrap() {

                                winnings += &bets[index].wager;
                                bettotal += &bets[index].wager;
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }
                

            }
            _ => {
                return Err(StdError::generic_err("invalid bet"));
            }
        }

        index += 1;
    }

    if totalamount != amount.u128() {
        return Err(StdError::generic_err("invalid amount"));
    }


    let sendback = (winnings + bettotal) * 1000000;
    let jackpot_send = amount.u128() / 50;

    state.jackpot += jackpot_send;
    STATE.save(deps.storage, &state).unwrap();

    let roulette_history = LastSpin {
        winning_number: spin,
        bets: bets,
        winnings: sendback,
    };
    LASTSPIN.insert(deps.storage, &from, &roulette_history).unwrap();

    if sendback > 0 {

        let msg = to_binary(&Snip20Msg::transfer_snip(
            from,
            sendback.into(),
        ))?;
        let message = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: state.known_snip.to_string(),
            code_hash: state.snip_hash,
            msg,
            funds: vec![],
        });
        let response = Response::new()
        .add_attribute("random_number", spin.to_string())
        .add_attribute("winValue", winnings.to_string())
        .add_attribute("betTotal", bettotal.to_string())
        .add_message(message);
        Ok(response)
    } else {
        let response = Response::new()
        .add_attribute("random_number", spin.to_string())
        .add_attribute("winValue", winnings.to_string())
        .add_attribute("betTotal", bettotal.to_string());
        Ok(response)
    }
}