use clap::Parser;
use std::{fmt, fs::File, io::Read};

// Parsing command line arguments
/// A rusty implementation of Tic Tac Toe.
#[derive(Parser)]
pub struct Cli {
    /// Player X
    #[clap(short = 'x', long = "player-x")]
    pub player_x: String,
    /// Player O
    #[clap(short = 'o', long = "player-o")]
    pub player_o: String,
    /// No. of times to play
    #[clap(default_value_t = 1, short = 'n')]
    pub repeat: u32,
}

/// Available players
pub enum Player {
    Human,
    Easy,
    Medium,
    Hard,
    Unbeatable, // TODO: Implement unbeatable AI
}

#[derive(Debug, Clone)]
struct Cell {
    cell_state: CellState,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            cell_state: CellState::Empty,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    X,
    O,
    Empty,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellState::X => write!(f, "X"),
            CellState::O => write!(f, "O"),
            CellState::Empty => write!(f, " "),
        }
    }
}

impl CellState {
    /// Returns the opponent
    pub fn opposite(self) -> CellState {
        match self {
            CellState::X => CellState::O,
            CellState::O => CellState::X,
            CellState::Empty => CellState::Empty,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<Cell>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: vec![Cell::new(); 9],
        }
    }

    pub fn load_from_file(filename: &str) -> Board {
        let mut board = Board::new();
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        for (i, line) in contents.lines().enumerate() {
            for (j, cell) in line.chars().enumerate() {
                match cell {
                    'X' => board.cells[i * 3 + j].cell_state = CellState::X,
                    'O' => board.cells[i * 3 + j].cell_state = CellState::O,
                    _ => board.cells[i * 3 + j].cell_state = CellState::Empty,
                }
            }
        }
        board
    }

    pub fn get_cell(&self, index: usize) -> CellState {
        self.cells[index].cell_state
    }

    pub fn set_cell(&mut self, index: usize, cell_state: CellState) {
        self.cells[index].cell_state = cell_state;
    }

    /// Get the winner of the game
    /// Returns the winner if there is one, None otherwise
    pub fn get_winner(&self) -> Option<CellState> {
        let mut winner: Option<CellState> = None;
        let mut winning_cells_x: Vec<usize> = vec![];
        let mut winning_cells_o: Vec<usize> = vec![];
        let winning_combinations: Vec<Vec<usize>> = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![0, 3, 6],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![0, 4, 8],
            vec![2, 4, 6],
        ];
        for combination in winning_combinations {
            winning_cells_x.clear();
            winning_cells_o.clear();
            for index in combination {
                if self.get_cell(index) == CellState::Empty {
                    break;
                } else if self.get_cell(index) == CellState::X {
                    winning_cells_x.push(index);
                } else {
                    winning_cells_o.push(index);
                }
            }
            if winning_cells_x.len() == 3 {
                winner = Some(CellState::X);
                break;
            } else if winning_cells_o.len() == 3 {
                winner = Some(CellState::O);
                break;
            }
        }
        winner
    }

    pub fn is_full(&self) -> bool {
        for cell in self.cells.iter() {
            if cell.cell_state == CellState::Empty {
                return false;
            }
        }
        true
    }

    /// Get legal moves for the player
    pub fn get_legal_moves(&self) -> Vec<usize> {
        let mut legal_moves: Vec<usize> = vec![];
        for index in 0..9 {
            if self.get_cell(index) == CellState::Empty {
                legal_moves.push(index);
            }
        }
        legal_moves
    }

    /// Checks if the move is valid
    /// i.e. the cell is empty and the move is within the board
    ///
    /// Returns true if the move is valid, false otherwise
    pub fn is_valid_move(&self, move_coord: (usize, usize)) -> bool {
        let index: usize = move_coord.0 + move_coord.1 * 3;
        if move_coord.0 > 2 || move_coord.1 > 2 {
            return false;
        }
        if self.get_cell(index) == CellState::Empty {
            return true;
        }
        false
    }
}

/// Renders the board onto the terminal
pub fn render(board: &Board) {
    println!();
    println!("    0 | 1 | 2 ");
    println!("  +---+---+---+");
    println!(
        "0 | {} | {} | {} |",
        board.get_cell(0),
        board.get_cell(1),
        board.get_cell(2)
    );
    println!("  +---+---+---+");
    println!(
        "1 | {} | {} | {} |",
        board.get_cell(3),
        board.get_cell(4),
        board.get_cell(5)
    );
    println!("  +---+---+---+");
    println!(
        "2 | {} | {} | {} |",
        board.get_cell(6),
        board.get_cell(7),
        board.get_cell(8)
    );
    println!("  +---+---+---+");
    println!();
}

/// Get the player's move from the user
pub fn get_player_move(_board: &Board, player_turn: CellState) -> (usize, usize) {
    println!("Player {}'s move ", player_turn);

    // Get Y coordinate of the move
    println!("What is move's Y coordinate?: ");
    let mut move_str = String::new();
    std::io::stdin().read_line(&mut move_str).unwrap();
    let move_str = move_str.trim();
    let y_coord = move_str.parse::<usize>().unwrap();

    // Get X coordinate of the move
    println!("What is move's X coordinate?: ");
    let mut move_str = String::new();
    std::io::stdin().read_line(&mut move_str).unwrap();
    let move_str = move_str.trim();
    let x_coord = move_str.parse::<usize>().unwrap();

    (x_coord, y_coord)
}

/// Executes the move on the board
pub fn make_move(board: &Board, move_coord: (usize, usize), player: CellState) -> Board {
    let mut new_board: Board = Board::clone(&board);
    let index: usize = move_coord.0 + move_coord.1 * 3;
    new_board.set_cell(index, player);
    new_board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winner_fn_returns_none_when_no_winner() {
        let board = Board::load_from_file("test_files/test_board_2.txt");
        assert_eq!(board.get_winner(), None);
    }

    #[test]
    fn winner_fn_returns_the_correct_winner() {
        let board = Board::load_from_file("test_files/test_board_1.txt");
        assert_eq!(board.get_winner(), Some(CellState::X));
    }

    #[test]
    fn is_full_fn_returns_true_when_board_is_full() {
        let board = Board::load_from_file("test_files/test_board_3.txt");
        assert_eq!(board.is_full(), true);
    }
}
