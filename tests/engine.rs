use buscaminas_v2_rust::constants::CellState;
use buscaminas_v2_rust::engine::{Board, FlagAction, RevealResult};

#[test]
fn first_click_is_safe_and_keeps_requested_mine_count() {
    let mut board = Board::new(9, 9, 10);
    let result = board.reveal(4, 4, 1.0);

    assert!(!matches!(result, RevealResult::Mine(_, _)));
    assert!(!board.grid[board.idx(4, 4)].is_mine);
    assert_eq!(board.grid.iter().filter(|cell| cell.is_mine).count(), 10);

    for (row, col) in board.get_neighbors(4, 4) {
        assert!(!board.grid[board.idx(row, col)].is_mine);
    }
}

#[test]
fn flag_question_hidden_cycle_keeps_counter_consistent() {
    let mut board = Board::new(9, 9, 10);

    assert_eq!(board.toggle_flag(0, 0), FlagAction::Flag);
    assert_eq!(board.flags_count, 1);
    assert_eq!(board.grid[board.idx(0, 0)].state, CellState::Flagged);

    assert_eq!(board.toggle_flag(0, 0), FlagAction::Question);
    assert_eq!(board.flags_count, 0);
    assert_eq!(board.grid[board.idx(0, 0)].state, CellState::Question);

    assert_eq!(board.toggle_flag(0, 0), FlagAction::Unflag);
    assert_eq!(board.flags_count, 0);
    assert_eq!(board.grid[board.idx(0, 0)].state, CellState::Hidden);
}

#[test]
fn excessive_mine_count_is_clamped_to_leave_one_safe_cell() {
    let board = Board::new(2, 2, 99);
    assert_eq!(board.total_mines, 3);
}

#[test]
fn question_cell_can_be_revealed_directly() {
    let mut board = Board::new(3, 3, 1);
    let idx = board.idx(1, 1);
    board.grid[idx].state = CellState::Question;

    let result = board.reveal(1, 1, 1.0);

    assert!(!matches!(
        result,
        RevealResult::None | RevealResult::Mine(_, _)
    ));
    assert_eq!(board.grid[idx].state, CellState::Revealed);
}

#[test]
fn one_by_one_board_is_safe_and_does_not_panic() {
    let mut board = Board::new(1, 1, 1);
    assert_eq!(board.total_mines, 0);
    assert!(matches!(board.reveal(0, 0, 1.0), RevealResult::Win(_)));
}

#[test]
fn out_of_bounds_actions_are_ignored() {
    let mut board = Board::new(9, 9, 10);
    assert_eq!(board.reveal(9, 0, 1.0), RevealResult::None);
    assert_eq!(board.toggle_flag(0, 9), FlagAction::None);

    let _ = board.reveal(0, 0, 2.0);
    assert_eq!(board.chord(99, 99, 3.0), RevealResult::None);
}

#[test]
fn paused_time_is_excluded_from_elapsed_seconds() {
    let mut board = Board::new(9, 9, 10);
    let _ = board.reveal(4, 4, 10.0);

    board.pause(15.0);
    assert_eq!(board.elapsed_seconds(25.0), 5);

    board.resume(30.0);
    assert_eq!(board.elapsed_seconds(35.0), 10);
}
