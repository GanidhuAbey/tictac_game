fn main() {
    let st = " ".to_string();
    let mut game_board = vec![st.clone(),st.clone(),st.clone(), st.clone(),st.clone(),st.clone(), st.clone(),st.clone(),st.clone(),];

    display_board(&game_board);
}

fn display_board(display_board: &Vec<String>) {
    let mut limit = 2;
    for i in 0..display_board.len() {
        if i == limit {
            println!("");
            limit = limit + 3;
        }
        print!("|{}|", display_board[i]);
    }
}

fn manipulate_board() {
    
}

fn convert_board() {

}
