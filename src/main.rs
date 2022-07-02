use clap::Parser;

use ai::{finds_winning_and_losing_moves_ai, finds_winning_moves_ai, minimax_ai, random_ai};
use tictactoe::{get_player_move, make_move, render, Board, CellState, Cli, Player};

mod ai;

fn main() {
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
        "unbeatable" => Ok(Player::Unbeatable),
        _ => Err("Invalid player.".to_string()),
    }
    .unwrap();

    let repeat = args.repeat;

    let mut player_turn: CellState = CellState::X;
    let mut move_coord: (usize, usize);

    let mut x_wins: u32 = 0;
    let mut o_wins: u32 = 0;

    for _ in 0..repeat {
        // Create a new board
        let mut board: Board = Board::new();
        render(&board);

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
                        Player::Unbeatable => Ok(minimax_ai(&board, player_turn)),
                    },
                    CellState::O => match player_o {
                        Player::Human => Ok(get_player_move(&board, player_turn)),
                        Player::Easy => Ok(random_ai(&board, player_turn)),
                        Player::Medium => Ok(finds_winning_moves_ai(&board, player_turn)),
                        Player::Hard => Ok(finds_winning_and_losing_moves_ai(&board, player_turn)),
                        Player::Unbeatable => Ok(minimax_ai(&board, player_turn)),
                    },
                    CellState::Empty => {
                        // Shouldn't happen, but rust won't leave me alone.
                        Err("Hey, this isn't supposed to happen.")
                    }
                }
                .unwrap();

                if board.is_valid_move(move_coord) {
                    break;
                } else {
                    println!("Invalid move! Try again");
                    println!();
                }
            }

            board = make_move(&board, move_coord, player_turn);
            render(&board);

            let winner: Option<CellState> = board.get_winner();

            match winner {
                Some(CellState::X) => {
                    println!("Player X wins!");
                    println!();
                    x_wins += 1;
                    break;
                }
                Some(CellState::O) => {
                    println!("Player O wins!");
                    println!();
                    o_wins += 1;
                    break;
                }
                _ => {
                    if board.is_full() {
                        println!("It's a draw!");
                        println!();
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

    if repeat > 1 {
        println!("Player X won {} times.", x_wins);
        println!(
            "Player X win percentage: {}%",
            (x_wins as f64 / repeat as f64) * 100.0
        );
        println!("Player O won {} times.", o_wins);
        println!(
            "Player O win percentage: {}%",
            (o_wins as f64 / repeat as f64) * 100.0
        );
        println!("It's a draw {} times.", repeat - x_wins - o_wins);
        println!(
            "Draw percentage: {}%",
            ((repeat - x_wins - o_wins) as f64 / repeat as f64) * 100.0
        );
    }
}
