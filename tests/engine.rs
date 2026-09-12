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
