

mod tictactoe;

use std::io;
use crate::tictactoe::next_move;
use crate::tictactoe::win_loss;

fn main() {
    let st = " ".to_string();
    let mut game_board = vec![st.clone(),st.clone(),st.clone(), st.clone(),st.clone(),st.clone(), st.clone(),st.clone(),st.clone()];

    let mut game = true;
    let mut num_board: Vec<u32>;

    display_board(&game_board);
    while game {
        manipulate_board(&mut game_board);
        num_board = convert_board(&game_board);
        num_board = next_move(num_board, 1);
        convert_display(&num_board, &mut game_board);
        display_board(&game_board);

        let state = win_loss(&num_board);
        match state {
            -1 => {
                println!("WIN");
                game = false;
            }
            1 => {
                println!("LOSS");
                game = false;
            }
            0 => {
                println!("DRAW");
                game = false
            }
            _ => (),
        }
    }
}

fn display_board(display_board: &Vec<String>) {
    let mut limit = 2;
    for i in 0..display_board.len() {
        print!("|{}|", display_board[i]);
        if i == limit {
            println!("");
            limit = limit + 3;
        }
    }
    println!("");
}

fn manipulate_board(game_board: &mut Vec<String>) {
    let mut invalid_input = true;
    let mut place: i32 = 0;

    while invalid_input {
        let mut p_input = String::new();
        println!("please enter a number between 1 and 9 to place a move");
        io::stdin().read_line(&mut p_input)
            .expect("Failed to read line");


        place = p_input.trim().parse()
            .expect("you entered invalid input...");
        place = place - 1;

        if place >= 0 && place < 9 && game_board[place as usize] == " ".to_string() {
            invalid_input = false;
        }
    }
    game_board[place as usize] = "O".to_string();
}

fn convert_board(game_board: &Vec<String>) -> Vec<u32> {
    let mut num_board = Vec::new();
    for i in 0..game_board.len() {
        if game_board[i] == "O".to_string() {
            num_board.push(2);
        }
        else if game_board[i] == "X".to_string() {
            num_board.push(1);
        }
        else {
            num_board.push(0);
        }
    }
    return num_board
}

fn convert_display(num_board: &Vec<u32>, game_board: &mut Vec<String>) {
    for i in 0..num_board.len() {
        if num_board[i] == 1 {
            game_board[i] = "X".to_string();
        }
    }
}
