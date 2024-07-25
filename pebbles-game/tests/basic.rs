#[cfg(test)]
mod tests {
    use super::*;
    use gstd::{prelude::*};
    use gtest::{Program, System};
    use pebbles_game_io::*;

    fn create_system_and_user() -> (System, u64) {
        let sys = System::new();
        sys.init_logger();
        let user_id = 1;
        sys.mint_to(user_id, 90000000000000);
        (sys, user_id)
    }

    #[test]
    fn test_initialization() {
        let (sys, user_id) = create_system_and_user();
        let program = Program::current(&sys);

        // Initialize the game
        let init_message = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        };

        let res = program.send_bytes(user_id, init_message.encode());
        assert!(!res.log().is_empty(), "Initialization should not log anything");
    }

    #[test]
    fn test_user_turn() {
        let (sys, user_id) = create_system_and_user();
        let program = Program::current(&sys);

        // Initialize the game
        let init_message = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        };
        program.send_bytes(user_id, init_message.encode());

        // User takes a turn
        let action = PebblesAction::Turn(2);
        let res = program.send_bytes(user_id, action.encode());

        // Check the game state after user's turn
        let state: GameState = program.read_state(()).unwrap();
        assert_eq!(state.pebbles_remaining, 3, "Pebbles remaining should be 8 after user takes 2 pebbles");
    }

    #[test]
    fn test_user_give_up() {
        let (sys, user_id) = create_system_and_user();
        let program = Program::current(&sys);

        // Initialize the game
        let init_message = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        };
        program.send_bytes(user_id, init_message.encode());

        // User gives up
        let action = PebblesAction::GiveUp;
        let res = program.send_bytes(user_id, action.encode());

        // Check the game state after user gives up
        let state: GameState = program.read_state(()).unwrap();
        assert_eq!(state.winner, Some(Player::Program), "Program should win after user gives up");
    }

    #[test]
    fn test_restart_game() {
        let (sys, user_id) = create_system_and_user();
        let program = Program::current(&sys);

        // Initialize the game
        let init_message = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        };
        program.send_bytes(user_id, init_message.encode());

        // Restart the game
        let action = PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 15,
            max_pebbles_per_turn: 5,
        };
        let res = program.send_bytes(user_id, action.encode());

        // Check the game state after restart
        let state: GameState = program.read_state(()).unwrap();
        assert_eq!(state.pebbles_count, 15, "Pebbles count should be 15 after restart");
        assert_eq!(state.max_pebbles_per_turn, 5, "Max pebbles per turn should be 5 after restart");
    }
}
