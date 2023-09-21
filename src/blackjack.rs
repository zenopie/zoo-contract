use cosmwasm_std::{
    to_binary, DepsMut, Env, MessageInfo, Response, StdError, StdResult, CosmosMsg, Uint128,
    WasmMsg, Addr
};
use crate::operations::{try_draw_card, try_get_deck};
use crate::state::{STATE, BLACKJACK, Card, Blackjack};
use crate::msg::Snip20Msg;

pub fn try_blackjack(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: String,
) -> StdResult<Response> {

    let bj_state_option:Option<Blackjack> = BLACKJACK.get(deps.storage, &info.sender);
    if bj_state_option.is_none() {
        return Err(StdError::generic_err("no game state"));
    }
    let mut bj_state:Blackjack = bj_state_option.unwrap();

    let mut deck = bj_state.deck.clone();
    let mut result = "error".to_string();

    let state = STATE.load(deps.storage).unwrap();

    match action.as_str() {
        "hit" => {

            if bj_state.action.as_str() != "deal" && bj_state.action.as_str() != "play" {
                return Err(StdError::generic_err("invalid phase"));
            }

            //draw a card and add to card array
            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.player.push(c.clone());
            deck = d;

            //add up points
            let point_total = bj_find_point_total(bj_state.player.clone());

            if point_total > 21 {
                result = "lose".to_string();
                bj_state.action = "ready".to_string();
                bj_state.result = "lose".to_string();
            } else {
                result = "play".to_string();
                bj_state.action = "play".to_string()
            }
            bj_state.deck = deck;
            BLACKJACK.insert(deps.storage, &info.sender, &bj_state).unwrap();

            let response = Response::new()
            .add_attribute("result", result)
            .add_attribute("new_card", c.id.to_string());
            Ok(response)
        }
        "stand" => {

            if bj_state.action.as_str() != "deal" && bj_state.action.as_str() != "play" {
                return Err(StdError::generic_err("invalid phase"));
            }
            
            let mut winnings:u128 = 0;
            let player_point_total = bj_find_point_total(bj_state.player.clone());

            let (dealer_point_total, dealer_cards, d) = bj_resolve_dealer(env.clone(), bj_state.dealer, deck);
            bj_state.dealer = dealer_cards.clone();
            bj_state.deck = d;
            bj_state.action = "ready".to_string();
            
            let send_dealer = bj_return_id_string(dealer_cards);

            if dealer_point_total > 21 {
                result = "win".to_string();
                winnings = bj_state.wager * 2;
            } else {
                if dealer_point_total < player_point_total {
                    result = "win".to_string();
                    winnings = bj_state.wager * 2
                } else if dealer_point_total > player_point_total {
                    result = "lose".to_string();
                } else if dealer_point_total == player_point_total {
                    result = "push".to_string();
                    winnings = bj_state.wager
                }
            }
            bj_state.result = result.clone();
            bj_state.winnings = winnings;
            BLACKJACK.insert(deps.storage, &info.sender, &bj_state).unwrap();
            if winnings == 0 {
                let response = Response::new()
                .add_attribute("result", result)
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            } else {
                let msg = to_binary(&Snip20Msg::transfer_snip(
                    info.sender.clone(),
                    winnings.into(),
                ))?;
                let message = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: state.known_snip.to_string(),
                    code_hash: state.snip_hash,
                    msg,
                    funds: vec![],
                });
                let response = Response::new()
                .add_message(message)
                .add_attribute("result", result)
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            }
        }
        "splithitL" => {

            if bj_state.action.as_str() != "splitL" {
                return Err(StdError::generic_err("invalid_phase"));
            }

            //draw a card and add to card array
            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.player.push(c.clone());
            deck = d;

            let point_total = bj_find_point_total(bj_state.player.clone());

            if point_total > 21 {
                result = "lose".to_string();
                bj_state.action = "splitR".to_string();
                bj_state.split_result = "loss".to_string();
            } else {
                result = "play".to_string();
            }
            bj_state.deck = deck;
            BLACKJACK.insert(deps.storage, &info.sender, &bj_state).unwrap();

            let response = Response::new()
            .add_attribute("result", result)
            .add_attribute("new_card", c.id.to_string());
            Ok(response)

        }
        "splithitR" => {

            if bj_state.action.as_str() != "splitR" {
                return Err(StdError::generic_err("invalid phase"));
            }

            //draw a card and add to card array
            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.split.push(c.clone());
            deck = d;

            let point_total = bj_find_point_total(bj_state.split.clone());
            
            bj_state.deck = deck.clone();

            let mut winnings:u128 = 0;
            let mut send_dealer = "n/a".to_string();
            if point_total > 21 {

                bj_state.action = "ready".to_string();

                if bj_state.split_result.as_str() == "loss" {

                    result = "loss/loss".to_string();

                } else if bj_state.split_result.as_str() == "stand" {

                    send_dealer = bj_return_id_string(bj_state.dealer.clone());
                    let player_point_total = bj_find_point_total(bj_state.player.clone());

                    let (dealer_point_total, dealer_cards, d) = bj_resolve_dealer(env.clone(), bj_state.dealer, deck);
                    bj_state.dealer = dealer_cards.clone();
                    bj_state.deck = d;
                    bj_state.action = "ready".to_string();

                    if dealer_point_total > 21 {

                        result = "win/loss".to_string();
                        winnings = bj_state.wager * 2;

                    } else {
                        if dealer_point_total < player_point_total {
        
                            result = "win/loss".to_string();
                            winnings = bj_state.wager * 2;
                            
                        } else if dealer_point_total > player_point_total {
        
                            result = "loss/loss".to_string();
        
                        } else if dealer_point_total == player_point_total {
        
                            result = "push/loss".to_string();
                            winnings = bj_state.wager;
        
                        } else {
                            return Err(StdError::generic_err("unknown error"));
                        }
                    }
                } else {
                    return Err(StdError::generic_err("unknown error"));
                }
            } else {

                result = "play".to_string();
                
            }
            if result != "play".to_string() {
                bj_state.result = result.clone();
                bj_state.winnings = winnings;
            }
            BLACKJACK.insert(deps.storage, &info.sender, &bj_state).unwrap();
            if winnings > 0 {

                let msg = to_binary(&Snip20Msg::transfer_snip(
                    info.sender.clone(),
                    winnings.into()
                ))?;
                let message = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: state.known_snip.to_string(),
                    code_hash: state.snip_hash,
                    msg,
                    funds: vec![],
                });
                let response = Response::new()
                .add_message(message)
                .add_attribute("result", result)
                .add_attribute("new_card", c.id.to_string())
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            } else {
                let response = Response::new()
                .add_attribute("result", result)
                .add_attribute("new_card", c.id.to_string())
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            }
        }
        "splitstandL" => {

            bj_state.split_result = "stand".to_string();
            bj_state.action = "splitR".to_string();
            BLACKJACK.insert(deps.storage, &info.sender, &bj_state).unwrap();

            Ok(Response::default())
        }
        "splitstandR" => {

            let (dealer_point_total, dealer_cards, d) = bj_resolve_dealer(env.clone(), bj_state.dealer, deck);
            bj_state.dealer = dealer_cards.clone();
            bj_state.deck = d;
            bj_state.action = "ready".to_string();

            let mut winnings:u128 = 0;

            if dealer_point_total > 21 {
                if bj_state.split_result.as_str() == "loss" {
                    result = "loss/win".to_string();
                    winnings = bj_state.wager * 2;
                } else {
                    result = "win/win".to_string();
                    winnings = bj_state.wager * 4;
                }
                
            } else {
                let mut first_result = "loss".to_string();
                if bj_state.split_result.as_str() != "loss" {
                    let left_point_total = bj_find_point_total(bj_state.player.clone());
                    
                    if dealer_point_total < left_point_total {
                        first_result = "win".to_string();
                        winnings = bj_state.wager * 2;
                    } else if dealer_point_total == left_point_total {
                        first_result = "push".to_string();
                        winnings = bj_state.wager;
                    }
                }
                let mut second_result = "loss".to_string();
                let right_point_total = bj_find_point_total(bj_state.split.clone());
                
                if dealer_point_total < right_point_total {
                    second_result = "win".to_string();
                    winnings += bj_state.wager * 2;
                } else if dealer_point_total == right_point_total {
                    second_result = "push".to_string();
                    winnings += bj_state.wager;
                }
                result = (first_result + "/" + &second_result).to_string();
            }
            let send_dealer = bj_return_id_string(bj_state.dealer.clone());

            bj_state.result = result.clone();
            bj_state.winnings = winnings.clone();
            BLACKJACK.insert(deps.storage, &info.sender, &bj_state).unwrap();

            if winnings > 0 {
                let send_dealer = bj_return_id_string(bj_state.dealer.clone());

                let msg = to_binary(&Snip20Msg::transfer_snip(
                    info.sender.clone(),
                    winnings.into()
                ))?;
                let message = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: state.known_snip.to_string(),
                    code_hash: state.snip_hash,
                    msg,
                    funds: vec![],
                });
                let response = Response::new()
                .add_message(message)
                .add_attribute("result", result)
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            } else {
                let response = Response::new()
                .add_attribute("result", result)
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            }
        }
        "surrender" => {

            if bj_state.action.as_str() != "deal" {
                return Err(StdError::generic_err("invalid phase"));
            }
            bj_state.action = "ready".to_string();
            bj_state.result = "surrender".to_string();
            bj_state.winnings = bj_state.wager / 2;
            BLACKJACK.insert(deps.storage, &info.sender, &bj_state).unwrap();

            let msg = to_binary(&Snip20Msg::transfer_snip(
                info.sender.clone(),
                (bj_state.wager / 2).into()
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
        "pass" => {
            if bj_state.action.as_str() != "insurance" {
                return Err(StdError::generic_err("invalid phase"));
            }
            let dealer_point_total = bj_find_point_total(bj_state.dealer.clone());

            bj_state.action = "deal".to_string();
            let mut result = "no dealer blackjack".to_string();
            let mut winnings:u128 = 0;
            if dealer_point_total == 21 {
                bj_state.action = "ready".to_string();
                BLACKJACK.insert(deps.storage, &info.sender, &bj_state).unwrap();
                let send_dealer = bj_return_id_string(bj_state.dealer.clone());
                let player_point_total = bj_find_point_total(bj_state.player.clone());
                result = "dealer blackjack".to_string();
                if player_point_total == 21 {
                    winnings = bj_state.wager;
                    result = "push".to_string();
                }
                let state = STATE.load(deps.storage).unwrap();
                let msg = to_binary(&Snip20Msg::transfer_snip(
                    info.sender.clone(),
                    winnings.into(),
                ))?;
                let message = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: state.known_snip.to_string(),
                    code_hash: state.snip_hash,
                    msg,
                    funds: vec![],
                });
                let response = Response::new()
                .add_message(message)
                .add_attribute("result", result)
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            } else {
                BLACKJACK.insert(deps.storage, &info.sender, &bj_state).unwrap();
                let response = Response::new()
                .add_attribute("result", result)
                .add_attribute("dealer_cards", "n/a".to_string());
                Ok(response)
            }
        }
        _ => {
            return Err(StdError::generic_err("invalid action"));
        }
    }
}



pub fn blackjack_receive(
    deps: DepsMut,
    env: Env,
    from: Addr,
    amount: Uint128,
    action: String,
) -> StdResult<Response> {

    let mut state = STATE.load(deps.storage).unwrap();
    if amount > state.max_bet {
        return Err(StdError::generic_err("above max bet"));
    }

    let jackpot_send = amount.u128() / 50;
    state.jackpot += jackpot_send;
    STATE.save(deps.storage, &state).unwrap();
    
    let mut bj_state_option:Option<Blackjack> = BLACKJACK.get(deps.storage, &from);
    if bj_state_option.is_none() {
        bj_state_option = Some(Blackjack{
            action: "ready".to_string(),
            dealer: Vec::new(), 
            player : Vec::new(),
            split: Vec::new(),
            split_result: "nosplit".to_string(),
            deck: Vec::new(),
            wager: 0,
            result: "in progress".to_string(),
            winnings: 0,
        })
    }
    let mut bj_state:Blackjack = bj_state_option.unwrap();

    match action.as_str() {
        "deal" => {
            
            if bj_state.action.as_str() != "ready" {
                return Err(StdError::generic_err("invalid phase"));
            }
            
            
            let mut deck = try_get_deck();
            bj_state.player = Vec::new();
            bj_state.dealer = Vec::new();
            bj_state.split = Vec::new();
            bj_state.split_result = "nosplit".to_string();
            bj_state.result = "in progress".to_string();

            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.player.push(c.clone());
            deck = d;

            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.dealer.push(c.clone());
            deck = d;

            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.player.push(c.clone());
            deck = d;

            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.dealer.push(c.clone());
            let mut send_dealer = c.id.to_string();
            deck = d;
            
            bj_state.action = "deal".to_string();
            bj_state.deck = deck;
            bj_state.wager = amount.u128();
            let mut result = "deal".to_string();
            if c.number == 10 || c.number == 11 {
                result = "insurance".to_string();
                bj_state.action = "insurance".to_string();
            }
            let mut winnings:u128 = 0;
            let player_point_total = bj_find_point_total(bj_state.player.clone());
            if player_point_total == 21 {
                bj_state.action = "ready".to_string();
                let dealer_point_total = bj_find_point_total(bj_state.dealer.clone());
                if dealer_point_total == 21 {
                    winnings = bj_state.wager;
                    result = "push".to_string();
                } else {
                    winnings = (bj_state.wager * 5) / 2;
                    result = "blackjack".to_string();
                }
            }
            let send_player = bj_return_id_string(bj_state.player.clone());
            if winnings == 0 {
                BLACKJACK.insert(deps.storage, &from, &bj_state).unwrap();
                let response = Response::new()
                .add_attribute("player_cards", send_player)
                .add_attribute("dealer_cards", send_dealer)
                .add_attribute("result", result);
                Ok(response)
            } else {
                bj_state.result = result.clone();
                bj_state.winnings = winnings.clone();
                BLACKJACK.insert(deps.storage, &from, &bj_state).unwrap();
                send_dealer = bj_return_id_string(bj_state.dealer.clone());

                let msg = to_binary(&Snip20Msg::transfer_snip(
                    from.clone(),
                    winnings.into()
                ))?;
                let message = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: state.known_snip.to_string(),
                    code_hash: state.snip_hash,
                    msg,
                    funds: vec![],
                });
                let response = Response::new()
                .add_message(message)
                .add_attribute("player_cards", send_player)
                .add_attribute("dealer_cards", send_dealer)
                .add_attribute("result", result);
                Ok(response)
            }   
        }
        "split" => {
            if amount.u128() != bj_state.wager {
                return Err(StdError::generic_err("invalid split amount"));
            }
            if bj_state.action.as_str() != "deal" {
                return Err(StdError::generic_err("invalid phase"));
            }
            if bj_state.player[0].number != bj_state.player[1].number {
                return Err(StdError::generic_err("cannot split"));
            }
            bj_state.split.push(bj_state.player[1].clone());
            bj_state.player.remove(1);

            let mut deck = bj_state.deck.clone();

            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.player.push(c.clone());
            deck = d;

            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.split.push(c.clone());
            deck = d;

            let send_left = bj_return_id_string(bj_state.player.clone());

            let send_right = bj_return_id_string(bj_state.split.clone());

            bj_state.deck = deck;
            bj_state.action = "splitL".to_string();
            bj_state.split_result = "inprogress".to_string();
            BLACKJACK.insert(deps.storage, &from, &bj_state).unwrap();

            let response = Response::new()
            .add_attribute("left_split", send_left)
            .add_attribute("right_split", send_right);
            Ok(response)
        }
        "double_down" => {
            if amount.u128() != bj_state.wager {
                return Err(StdError::generic_err("invalid amount"));
            }
            if bj_state.action.as_str() != "deal" {
                return Err(StdError::generic_err("invalid phase"));
            }
            let mut player_point_total = bj_find_point_total(bj_state.player.clone());
            if player_point_total != 10 && player_point_total != 11 {
                return Err(StdError::generic_err("only can double down on 10 or 11"));
            }


            let mut deck = bj_state.deck.clone();

            let (c, d) = try_draw_card(env.clone(), deck);
            bj_state.player.push(c.clone());
            deck = d;
            let new_card = c.id.to_string();
            player_point_total = bj_find_point_total(bj_state.player.clone());


            let (dealer_point_total, dealer_cards, d) = bj_resolve_dealer(env.clone(), bj_state.dealer, deck);
            bj_state.dealer = dealer_cards.clone();
            bj_state.deck = d;
            bj_state.action = "ready".to_string();

            let send_dealer = bj_return_id_string(dealer_cards);
            let mut winnings = 0u128;
            let result;

            if dealer_point_total > 21 {

                result = "win".to_string();
                winnings = bj_state.wager * 4;

            } else {
                
                if dealer_point_total < player_point_total {

                    result = "win".to_string();
                    winnings = bj_state.wager * 4;

                } else if dealer_point_total > player_point_total {

                    result = "lose".to_string();

                } else {

                    result = "push".to_string();
                    winnings = bj_state.wager * 2;
                }
            }
            bj_state.result = result.clone();
            bj_state.winnings = winnings.clone();
            BLACKJACK.insert(deps.storage, &from, &bj_state).unwrap();
            
            if winnings == 0 {
                let response = Response::new()
                .add_attribute("new_card", new_card)
                .add_attribute("result", result)
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            } else {
                let msg = to_binary(&Snip20Msg::transfer_snip(
                    from.clone(),
                    winnings.into(),
                ))?;
                let message = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: state.known_snip.to_string(),
                    code_hash: state.snip_hash,
                    msg,
                    funds: vec![],
                });
                let response = Response::new()
                .add_message(message)
                .add_attribute("new_card", new_card)
                .add_attribute("result", result)
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            }
        }
        "insurance" => {
            if amount.u128() != (bj_state.wager / 2) {
                return Err(StdError::generic_err("invalid amount"));
            }
            if bj_state.action.as_str() != "insurance" {
                return Err(StdError::generic_err("invalid phase"));
            }
            let dealer_point_total = bj_find_point_total(bj_state.dealer.clone());

            bj_state.action = "deal".to_string();
            let mut result = "insurance loss".to_string();
            
            if dealer_point_total == 21 {
                bj_state.action = "ready".to_string();
                BLACKJACK.insert(deps.storage, &from, &bj_state).unwrap();
                let mut winnings:u128 = bj_state.wager;
                let send_dealer = bj_return_id_string(bj_state.dealer.clone());
                let player_point_total = bj_find_point_total(bj_state.player.clone());
                result = "insurance win".to_string();
                if player_point_total == 21 {
                    winnings += bj_state.wager;
                    result = "insurance win/push".to_string();
                }
                let msg = to_binary(&Snip20Msg::transfer_snip(
                    from.clone(),
                    winnings.into(),
                ))?;
                let message = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: state.known_snip.to_string(),
                    code_hash: state.snip_hash,
                    msg,
                    funds: vec![],
                });
                let response = Response::new()
                .add_message(message)
                .add_attribute("result", result)
                .add_attribute("dealer_cards", send_dealer);
                Ok(response)
            } else {
                BLACKJACK.insert(deps.storage, &from, &bj_state).unwrap();
                let response = Response::new()
                .add_attribute("result", result)
                .add_attribute("dealer_cards", "n/a".to_string());
                Ok(response)
            }
        }
        _ => {
            return Err(StdError::generic_err("invalid action"));
        }
    }
}

pub fn bj_find_point_total(
    cards: Vec<Card>,
) -> u32 {
    //add up points
    let mut i = 0;
    let mut point_total = 0;
    while i < cards.len() {
        point_total += &cards[i].number;
        i += 1;
    }
    // look for aces if over 21
    i = 0;
    while i < cards.len() && point_total > 21 {
        if  cards[i].number == 11 {
            point_total -= 10;
        }
        i += 1;
    }
    return point_total;
}


pub fn bj_return_id_string(
    cards: Vec<Card>,
 ) -> String {

    let mut card_id_array = Vec::new();

    //add up points
    let mut i = 0;
    while i < cards.len() {
        card_id_array.push(cards[i].id);
        i += 1;
    }
    let send_ids = card_id_array
    .into_iter()
    .map(|c| c.to_string())
    .collect::<Vec<String>>()
    .join(",");

    return send_ids;
}

pub fn bj_resolve_dealer(
    env: Env,
    mut cards: Vec<Card>,
    mut deck: Vec<Card>,
 ) -> (u32, Vec<Card>, Vec<Card>)  {

    let mut dealer_point_total = bj_find_point_total(cards.clone());
             
    while dealer_point_total < 17 {

        let (c, d) = try_draw_card(env.clone(), deck);
        cards.push(c.clone());
        deck = d;
        dealer_point_total += c.number;

        if dealer_point_total > 21 {
            let mut i = 0;
            while i < cards.len() && dealer_point_total > 21 {
                if cards[i].number == 11 {
                    cards[i].number = 1;
                    dealer_point_total -= 10;
                }
                i += 1;
            }
        }  
    }
    return (dealer_point_total, cards, deck);
}
