use gtest::{Program, System};
use pebbles_game_io::*;

#[test]

fn test() {
    let mut system = System::new();
    let mut program = Program::new();

    system.init(
        DifficultyLevel::Easy,
        10,
        3,
    );

    assert_eq!(
        system.state(),
        GameState {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            pebbles_remaining: 10,
            difficulty: DifficultyLevel::Easy,
        }
    );

    assert_eq!(
        system.handle(PebblesAction::Turn(2)),
        PebblesEvent::CounterTurn(3),
    );

    assert_eq!(
        system.handle(PebblesAction::Turn(3)),
        PebblesEvent::Won(Player::User),
    );

    system.init(
        DifficultyLevel::Hard,
        10,
        3,
    );

    assert_eq!(
        system.state(),
        GameState {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            pebbles_remaining: 10,
            difficulty: DifficultyLevel::Hard,
        }
    );

    assert_eq!(
        system.handle(PebblesAction::Turn(4)),
        PebblesEvent::CounterTurn(3),
    );

    assert_eq!(
        system.handle(PebblesAction::Turn(3)),
        PebblesEvent::CounterTurn(3),
    );

    assert_eq!(
        system.handle(PebblesAction::Turn(3)),
        PebblesEvent::Won(Player::User),
    );

    assert_eq!(
        system.handle(PebblesAction::Restart {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        }),
        PebblesEvent::CounterTurn(3),
    );

    assert_eq!(
        system.state(),
        GameState {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            pebbles_remaining: 10,
            difficulty: DifficultyLevel::Easy,
        }
    );

    assert_eq!(
        system.handle(PebblesAction::Turn(2)),
        PebblesEvent::CounterTurn(3),
    );

    assert_eq!(
        system.handle(PebblesAction::Turn(3)),
        PebblesEvent::Won(Player::User),
    );

    assert_eq!(
        system.handle(PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        }),
        PebblesEvent::CounterTurn(3),
    );

}