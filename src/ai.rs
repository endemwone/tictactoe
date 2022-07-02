use tictactoe::{make_move, Board, CellState};

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

/// Returns the score using minimax algorithm
fn minimax_score(board: &Board, player_turn: CellState, player_to_optimize: CellState) -> i32 {
    let winner = board.get_winner().unwrap_or_else(|| CellState::Empty);

    if winner == player_to_optimize {
        return 10;
    } else if winner == player_to_optimize.opposite() {
        return -10;
    } else if board.is_full() {
        return 0;
    } else {
        // If board is not in a terminal state,
        // get all the moves that can be played.

        let legal_moves = board.get_legal_moves();

        // Iterate through these moves, calculating a score
        // for each one and adding it to the `scores` array.
        let mut scores: Vec<i32> = Vec::new();

        for index in legal_moves {
            let move_coord = (index % 3, index / 3);

            // Create a copy of the board and make the move.
            let mut new_board = Board::clone(board);
            new_board = make_move(&new_board, move_coord, player_turn);

            // Get the minimax score for the resulting state,
            // passing in current player's opponent because
            // it's their turn now.
            let opponent = player_turn.opposite();
            let opponent_best_response_score =
                minimax_score(&new_board, opponent, player_to_optimize);
            scores.push(opponent_best_response_score);
        }

        if player_turn == player_to_optimize {
            return *scores.iter().max().unwrap();
        } else {
            return *scores.iter().min().unwrap();
        }
    }
}

/// The unbeatable AI using minimax algorithm.
pub fn minimax_ai(board: &Board, player_turn: CellState) -> (usize, usize) {
    let mut best_move: Option<(usize, usize)> = None;
    let mut best_score: Option<i32> = None;

    let legal_moves = board.get_legal_moves();

    for index in legal_moves {
        let move_coord = (index % 3, index / 3);

        let mut new_board = Board::clone(board);
        new_board = make_move(&new_board, move_coord, player_turn);

        let opponent = player_turn.opposite();
        let score = minimax_score(&new_board, opponent, player_turn);
        if best_score == None || score > best_score.unwrap() {
            best_move = Some(move_coord);
            best_score = Some(score);
        }
    }

    best_move.unwrap()
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
