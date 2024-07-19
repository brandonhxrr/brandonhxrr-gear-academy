#![no_std]

use gstd::prelude::*;
use gstd::*;
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[no_mangle]
extern "C" fn init() {
    let PebblesInit {
        difficulty,
        pebbles_count,
        max_pebbles_per_turn,
    } = msg::load().expect("Failed to load PebblesInit");

    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    unsafe {
        PEBBLES_GAME = Some(GameState {
            pebbles_count,
            max_pebbles_per_turn,
            pebbles_remaining: pebbles_count,
            difficulty,
            first_player,
            winner: None,
        });
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Failed to load PebblesAction");

    unsafe {
        let game = PEBBLES_GAME.as_mut().unwrap();
        match action {
            PebblesAction::Turn(pebbles) => {
                if pebbles > game.max_pebbles_per_turn {
                    msg::reply(PebblesEvent::CounterTurn(game.max_pebbles_per_turn), 0).expect("Failed to send reply");
                    return;
                }
                game.pebbles_remaining -= pebbles;
                if game.pebbles_remaining == 0 {
                    game.winner = Some(Player::User);
                    msg::reply(PebblesEvent::Won(Player::User), 0).expect("Failed to send reply");
                } else {
                    msg::reply(PebblesEvent::CounterTurn(game.max_pebbles_per_turn), 0).expect("Failed to send reply");
                }
            }
            PebblesAction::GiveUp => {
                game.winner = Some(Player::Program);
                msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Failed to send reply");
            }
            PebblesAction::Restart {
                difficulty,
                pebbles_count,
                max_pebbles_per_turn,
            } => {
                let first_player = if get_random_u32() % 2 == 0 {
                    Player::User
                } else {
                    Player::Program
                };
                *game = GameState {
                    pebbles_count,
                    max_pebbles_per_turn,
                    pebbles_remaining: pebbles_count,
                    difficulty,
                    first_player,
                    winner: None,
                };
                msg::reply(PebblesEvent::CounterTurn(game.max_pebbles_per_turn), 0).expect("Failed to send reply");
            }
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let game_state = unsafe { PEBBLES_GAME.as_ref().unwrap().clone() };
    msg::reply(game_state, 0).expect("Failed to send reply");
}
