use cosmwasm_std::{Env, Response, StdResult, Uint128, Addr, DepsMut,};


use crate::state::{Card, ADMIN};

pub fn try_get_deck() -> Vec<Card> {
    let original_deck: Vec<Card> = vec![
        Card {
            suit: "clubs".to_string(),
            number: 11, 
            face: "ace".to_string(),
            id: 0,     
        },
        Card {
            suit: "clubs".to_string(),
            number: 2, 
            face: "number".to_string(),
            id: 1,   
        },
        Card {
            suit: "clubs".to_string(),
            number: 3,
            face: "number".to_string(),
            id: 2,
        },
        Card {
            suit: "clubs".to_string(),
            number: 4,    
            face: "number".to_string(),
            id: 3,
        },
        Card {
            suit: "clubs".to_string(),
            number: 5,  
            face: "number".to_string(),
            id: 4,
        },
        Card {
            suit: "clubs".to_string(),
            number: 6,    
            face: "number".to_string(),
            id: 5, 
        },
        Card {
            suit: "clubs".to_string(),
            number: 7,  
            face: "number".to_string(),
            id: 6,    
        },
        Card {
            suit: "clubs".to_string(),
            number: 8,   
            face: "number".to_string(),
            id: 7,    
        },
        Card {
            suit: "clubs".to_string(),
            number: 9,  
            face: "number".to_string(),
            id: 8,     
        },
        Card {
            suit: "clubs".to_string(),
            number: 10,  
            face: "number".to_string(),
            id: 9,     
        },
        Card {
            suit: "clubs".to_string(),
            number: 10,   
            face: "jack".to_string(),
            id: 10,
        },
        Card {
            suit: "clubs".to_string(),
            number: 10,  
            face: "king".to_string(),
            id: 11, 
        },
        Card {
            suit: "clubs".to_string(),
            number: 10,  
            face: "queen".to_string(),
            id: 12,
        },
        Card {
            suit: "diamonds".to_string(),
            number: 11,
            face: "ace".to_string(),
            id: 13,           
        },
        Card {
            suit: "diamonds".to_string(),
            number: 2,  
            face: "number".to_string(),
            id: 14,
        },
        Card {
            suit: "diamonds".to_string(),
            number: 3, 
            face: "number".to_string(),
            id: 15,
        },
        Card {
            suit: "diamonds".to_string(),
            number: 4,   
            face: "number".to_string(),
            id: 16,   
        },
        Card {
            suit: "diamonds".to_string(),
            number: 5,    
            face: "number".to_string(),
            id: 17,
        },
        Card {
            suit: "diamonds".to_string(),
            number: 6,  
            face: "number".to_string(),
            id: 18,
        },
        Card {
            suit: "diamonds".to_string(),
            number: 7, 
            face: "number".to_string(),
            id: 19, 
        },
        Card {
            suit: "diamonds".to_string(),
            number: 8,  
            face: "number".to_string(),
            id: 20,     
        },
        Card {
            suit: "diamonds".to_string(),
            number: 9,  
            face: "number".to_string(),
            id: 21,   
        },
        Card {
            suit: "diamonds".to_string(),
            number: 10,
            face: "number".to_string(),
            id: 22,       
        },
        Card {
            suit: "diamonds".to_string(),
            number: 10,   
            face: "jack".to_string(),
            id: 23,
        },
        Card {
            suit: "diamonds".to_string(),
            number: 10,
            face: "king".to_string(),
            id: 24,    
        },
        Card {
            suit: "diamonds".to_string(),
            number: 10,  
            face: "queen".to_string(),
            id: 25,
        },
        Card {
            suit: "hearts".to_string(),
            number: 11,   
            face: "ace".to_string(),
            id: 26,       
        },
        Card {
            suit: "hearts".to_string(),
            number: 2,   
            face: "number".to_string(),
            id: 27, 
        },
        Card {
            suit: "hearts".to_string(),
            number: 3,    
            face: "number".to_string(),
            id: 28,
        },
        Card {
            suit: "hearts".to_string(),
            number: 4,    
            face: "number".to_string(),
            id: 29,
        },
        Card {
            suit: "hearts".to_string(),
            number: 5,    
            face: "number".to_string(),
            id: 30,
        },
        Card {
            suit: "hearts".to_string(),
            number: 6, 
            face: "number".to_string(),
            id: 31,
        },
        Card {
            suit: "hearts".to_string(),
            number: 7,     
            face: "number".to_string(),
            id: 32,
        },
        Card {
            suit: "hearts".to_string(),
            number: 8,    
            face: "number".to_string(),
            id: 33,
        },
        Card {
            suit: "hearts".to_string(),
            number: 9,  
            face: "number".to_string(),
            id: 34,
        },
        Card {
            suit: "hearts".to_string(),
            number: 10,
            face: "number".to_string(),
            id: 35,
        },
        Card {
            suit: "hearts".to_string(),
            number: 10,  
            face: "jack".to_string(),
            id: 36,
        },
        Card {
            suit: "hearts".to_string(),
            number: 10,    
            face: "king".to_string(),
            id: 37,
        },
        Card {
            suit: "hearts".to_string(),
            number: 10,    
            face: "queen".to_string(),
            id: 38,
        },
        Card {
            suit: "spades".to_string(),
            number: 11,   
            face: "ace".to_string(),
            id: 39,
        },
        Card {
            suit: "spades".to_string(),
            number: 2,  
            face: "number".to_string(),
            id: 40,
        },
        Card {
            suit: "spades".to_string(),
            number: 3,  
            face: "number".to_string(),
            id: 41,
        },
        Card {
            suit: "spades".to_string(),
            number: 4,   
            face: "number".to_string(),
            id: 42,
        },
        Card {
            suit: "spades".to_string(),
            number: 5,   
            face: "number".to_string(),
            id: 43,
        },
        Card {
            suit: "spades".to_string(),
            number: 6,   
            face: "number".to_string(),
            id: 44,
        },
        Card {
            suit: "spades".to_string(),
            number: 7,  
            face: "number".to_string(),
            id: 45,
        },
        Card {
            suit: "spades".to_string(),
            number: 8, 
            face: "number".to_string(),
            id: 46,
        },
        Card {
            suit: "spades".to_string(),
            number: 9,   
            face: "number".to_string(),
            id: 47,
        },
        Card {
            suit: "spades".to_string(),
            number: 10,  
            face: "number".to_string(),
            id: 48,
        },
        Card {
            suit: "spades".to_string(),
            number: 10,    
            face: "jack".to_string(),
            id: 49,
        },
        Card {
            suit: "spades".to_string(),
            number: 10,    
            face: "king".to_string(),
            id: 50,
        },
        Card {
            suit: "spades".to_string(),
            number: 10,    
            face: "queen".to_string(),
            id: 51,
        },
        
    ];
    return original_deck;
}

pub fn try_draw_card(env: Env, mut deck: Vec<Card>) -> (Card, Vec<Card>) {
    let random_binary = env.block.random.clone();
    let random_bytes = &random_binary.as_ref().unwrap().0;
        
    let random_number = u32::from_le_bytes([
        random_bytes[0],
        random_bytes[1],
        random_bytes[2],
        random_bytes[3],
    ]);
    let spin = random_number % deck.len() as u32;

    let card = deck[spin as usize].clone();
    deck.remove(spin as usize);
    return (card.clone(), deck)
}

pub fn deposit_receive(
    deps: DepsMut,
    _env: Env,
    _from: Addr,
    amount: Uint128,
) -> StdResult<Response> {

    let mut admin = ADMIN.load(deps.storage).unwrap();
    admin.vault += amount.u128();
    ADMIN.save(deps.storage, &admin).unwrap();

    Ok(Response::default())
}