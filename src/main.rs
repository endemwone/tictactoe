use tictactoe::{get_player_move, is_valid_move, make_move, render, Board, CellState};

fn main() {
    let mut board: Board = Board::new();
    render(&board);

    let mut player_turn: CellState = CellState::X;
    let mut move_coord: (usize, usize);

    loop {
        // Loop to get player move
        loop {
            move_coord = get_player_move(player_turn);
            if is_valid_move(&board, move_coord) {
                break;
            } else {
                println!("Invalid move! Try again");
                println!();
            }
        }

        board = make_move(board, move_coord, player_turn);
        render(&board);

        let winner: Option<CellState> = board.get_winner();

        match winner {
            Some(CellState::X) | Some(CellState::O) => {
                println!("Player {} wins!", winner.unwrap());
                break;
            }
            _ => {
                if board.is_full() {
                    println!("Draw!");
                    break;
                }
            }
        }

        if player_turn == CellState::X {
            player_turn = CellState::O;
        } else {
            player_turn = CellState::X;
        }
    }
}
