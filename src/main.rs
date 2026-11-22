use chess::{Board, ChessMove};
use std::io::Write;
use std::str::FromStr;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_str(fen).expect("Failed to create board");
    println!("Welcome to the chess engine!");
    loop {
        println!("\nCurrent board:");
        println!("{}", board);
        print!("Enter your move (e.g., e2e4): ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "quit" {
            println!("Goodbye!");
            break;
        }
        
        match ChessMove::from_san(&board, input) {
            Ok(r#move) => {
                let mut new_board = Board::default();
                board.make_move(r#move, &mut new_board);
                board = new_board;
                println!("Move applied.");
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
