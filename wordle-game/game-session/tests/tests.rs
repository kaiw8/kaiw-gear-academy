use gtest::{Log, Program, System};
use game_session_io::*;

const MAX_ATTEMPTS: u32 = 6;
const TIMEOUT_DELAY: u32 = 1;
const PLAYER_ID: u64 = 42;

fn setup_program(sys: &System) -> Program {
    sys.init_logger();

    let game_session = Program::current(sys);
    let wordle_game = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/debug/wordle_game.opt.wasm",
    );

    assert_eq!(game_session.id(), 1.into());
    assert_eq!(wordle_game.id(), 2.into());

    let init_result_wordle = wordle_game.send_bytes(PLAYER_ID, b"");
    assert!(!init_result_wordle.main_failed());

    let init_result_session = game_session.send(
        PLAYER_ID,
        WordleInit {
            wordle_address: wordle_game.id(),
            count_attempts: MAX_ATTEMPTS,
            delay_timeout: TIMEOUT_DELAY,
        },
    );
    assert!(!init_result_session.main_failed());

    let current_state: WordleState = game_session.read_state(b"").unwrap();
    assert_eq!(current_state.count_attemps, MAX_ATTEMPTS);
    assert_eq!(current_state.status, WordleStatus::Init);

    game_session
}

#[test]
fn test_game_start() {
    let sys = System::new();
    let game_prog = setup_program(&sys);

    let game_start_result = game_prog.send(PLAYER_ID, WordleAction::StartGame);
    let expected_log = Log::builder().payload(WordleEvent::GameStartSuccess);
    assert!(game_start_result.contains(&expected_log));

    let current_state: WordleState = game_prog.read_state(b"").unwrap();
    assert_eq!(current_state.status, WordleStatus::GameStarted);
}

#[test]
fn test_word_check() {
    let sys = System::new();
    let game_prog = setup_program(&sys);
    let _ = game_prog.send(PLAYER_ID, WordleAction::StartGame);

    let word_check_result = game_prog.send(PLAYER_ID, WordleAction::CheckWord("hhhhh".to_string()));
    let expected_log = Log::builder().payload(WordleEvent::CheckWordSuccess(
        wordle_game_io::Event::WordChecked {
            user: PLAYER_ID.into(),
            correct_positions: vec![0],
            contained_in_word: vec![1, 2, 3, 4],
        },
    ));
    assert!(word_check_result.contains(&expected_log));
}

#[test]
fn test_player_win() {
    let sys = System::new();
    let game_prog = setup_program(&sys);
    let _ = game_prog.send(PLAYER_ID, WordleAction::StartGame);

    let test_words = vec!["house", "human", "horse"];
    let expected_log = Log::builder().payload(WordleEvent::YouAreWin);
    let results = test_words.into_iter().map(|word| game_prog.send(PLAYER_ID, WordleAction::CheckWord(word.to_string())));
    assert!(results.any(|result| result.contains(&expected_log)));
}

#[test]
fn test_player_loose() {
    let sys = System::new();
    let game_prog = setup_program(&sys);
    let _ = game_prog.send(PLAYER_ID, WordleAction::StartGame);

    let expected_log = Log::builder().payload(WordleEvent::YouAreLoose);
    let check_results = (0..(MAX_ATTEMPTS as usize))
        .map(|_| game_prog.send(PLAYER_ID, WordleAction::CheckWord("qwert".to_string())));
    assert!(check_results.last().unwrap().contains(&expected_log));
}

#[test]
fn test_game_timeout() {
    let sys = System::new();
    let game_prog = setup_program(&sys);
    let _ = game_prog.send(PLAYER_ID, WordleAction::StartGame);

    std::thread::sleep(std::time::Duration::from_secs(10));

    let final_state: WordleState = game_prog.read_state(b"").unwrap();
    assert_eq!(final_state.status, WordleStatus::GameOver(WordlePlayerStatus::Loose));
}
