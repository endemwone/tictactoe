use tictactoe::{Board, CellState};

use rand::seq::SliceRandom;

/// Returns random coordinates for the move
/// irrespective of the player's turn
pub fn random_ai(board: &Board, _player_turn: CellState) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let mut empty_cells: Vec<usize> = vec![];
    for index in 0..9 {
        if board.get_cell(index) == CellState::Empty {
            empty_cells.push(index);
        }
    }

    let index = empty_cells.choose(&mut rng).unwrap();
    let x_coord = index % 3;
    let y_coord = index / 3;
    (x_coord, y_coord)
}

/// Returns the coordinates of the winning move
/// if there is one, otherwise returns random move
pub fn finds_winning_moves_ai(board: &Board, player_turn: CellState) -> (usize, usize) {
    let mut empty_cells: Vec<usize> = vec![];
    for index in 0..9 {
        if board.get_cell(index) == CellState::Empty {
            empty_cells.push(index);
        }
    }

    for index in empty_cells {
        let mut new_board: Board = Board::clone(board);
        new_board.set_cell(index, player_turn);
        if new_board.get_winner() == Some(player_turn) {
            return (index % 3, index / 3);
        }
    }

    random_ai(board, player_turn)
}

/// Returns the coordinates of the winning move,
/// blocks losing move, otherwise returns random move
pub fn finds_winning_and_losing_moves_ai(board: &Board, player_turn: CellState) -> (usize, usize) {
    // Find empty cells
    let mut empty_cells: Vec<usize> = vec![];
    for index in 0..9 {
        if board.get_cell(index) == CellState::Empty {
            empty_cells.push(index);
        }
    }

    // Find winning move
    for index in &empty_cells {
        let mut new_board: Board = Board::clone(board);
        new_board.set_cell(*index, player_turn);
        if new_board.get_winner() == Some(player_turn) {
            return (index % 3, index / 3);
        }
    }

    // Blocks losing move
    for index in &empty_cells {
        let mut new_board: Board = Board::clone(board);
        new_board.set_cell(*index, player_turn.opposite());
        if new_board.get_winner() == Some(player_turn.opposite()) {
            return (index % 3, index / 3);
        }
    }

    // Returns random move
    random_ai(board, player_turn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_ai_returns_random_move() {
        let board = Board::new();
        let (x_coord, y_coord) = random_ai(&board, CellState::X);
        assert!(x_coord < 3);
        assert!(y_coord < 3);
    }

    #[test]
    fn finds_winning_moves_ai_returns_winning_move() {
        let board = Board::load_from_file("test_files/test_board_4.txt");
        assert_eq!(finds_winning_moves_ai(&board, CellState::X), (0, 1));
        assert_eq!(finds_winning_moves_ai(&board, CellState::O), (1, 2));
    }

    #[test]
    fn finds_winning_and_losing_moves_ai_returns_winning_move() {
        let board = Board::load_from_file("test_files/test_board_4.txt");
        assert_eq!(
            finds_winning_and_losing_moves_ai(&board, CellState::X),
            (0, 1)
        );
        assert_eq!(
            finds_winning_and_losing_moves_ai(&board, CellState::O),
            (1, 2)
        );
    }

    #[test]
    fn finds_winning_and_losing_moves_ai_blocks_losing_move() {
        let board = Board::load_from_file("test_files/test_board_5.txt");
        assert_eq!(
            finds_winning_and_losing_moves_ai(&board, CellState::O),
            (2, 0)
        );
    }
}
