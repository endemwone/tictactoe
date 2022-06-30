// Rust implementation of Tic Tac Toe
//
// # Usage
//
// Enter the required player against the player option.
//
// `-x` or `--player_x` for player X.
//
// `-o` or `--player_o` for player O.
//
// The player options are optional. If not specified, the program will pit you
// against a computer with random difficulty. (TODO)
//
// You should specify both players.
//
// ## Players
//
// `human` => Human player
//
// `easy` => Level 1: AI that makes random moves
//
// `medium` => Level 2: AI that finds the available winning moves
//
// `hard` => Level 3: AI that finds the available winning and losing moves
//
// `unbeatable` => Level 4: AI that makes perfect moves

use clap::Parser;

use ai::{finds_winning_and_losing_moves_ai, finds_winning_moves_ai, random_ai};
use tictactoe::{get_player_move, is_valid_move, make_move, render, Board, CellState, Cli, Player};

mod ai;

fn main() {
    // Create a new board
    let mut board: Board = Board::new();
    render(&board);

    // Parse the arguments and get the players
    let args = Cli::parse();
    let player_x = match args.player_x.as_str() {
        "human" => Ok(Player::Human),
        "easy" => Ok(Player::Easy),
        "medium" => Ok(Player::Medium),
        "hard" => Ok(Player::Hard),
        "unbeatable" => Ok(Player::Unbeatable),
        _ => Err("Invalid player.".to_string()),
    }
    .unwrap();
    let player_o = match args.player_o.as_str() {
        "human" => Ok(Player::Human),
        "easy" => Ok(Player::Easy),
        "medium" => Ok(Player::Medium),
        "hard" => Ok(Player::Hard),
        // TODO: Implement unbeatable AI
        // "unbeatable" => Ok(Player::Unbeatable),
        _ => Err("Invalid player.".to_string()),
    }
    .unwrap();

    let mut player_turn: CellState = CellState::X;
    let mut move_coord: (usize, usize);

    loop {
        // Loop to get player move
        loop {
            // Get player move from the player/AI
            move_coord = match player_turn {
                CellState::X => match player_x {
                    Player::Human => Ok(get_player_move(&board, player_turn)),
                    Player::Easy => Ok(random_ai(&board, player_turn)),
                    Player::Medium => Ok(finds_winning_moves_ai(&board, player_turn)),
                    Player::Hard => Ok(finds_winning_and_losing_moves_ai(&board, player_turn)),
                    // "unbeatable" => {
                    //     unbeatable_ai(&board, player_turn)
                    // }
                    _ => Err("Invalid player."),
                },
                CellState::O => match player_o {
                    Player::Human => Ok(get_player_move(&board, player_turn)),
                    Player::Easy => Ok(random_ai(&board, player_turn)),
                    Player::Medium => Ok(finds_winning_moves_ai(&board, player_turn)),
                    Player::Hard => Ok(finds_winning_and_losing_moves_ai(&board, player_turn)),
                    // "unbeatable" => {
                    //     unbeatable_ai(&board, player_turn)
                    // }
                    _ => Err("Invalid player."),
                },
                CellState::Empty => {
                    // Shouldn't happen, but rust won't leave me alone.
                    Err("Hey, this isn't supposed to happen.")
                }
            }
            .unwrap();

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
                    println!("It's a draw!");
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
