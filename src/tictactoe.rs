const COMPUTER: u32 = 1;
const PLAYER: u32 = 2;

const UNASSIGNED: i32 = 4; //indicating that this value is more or less undefined in terms of our program

//Ai decision tree program for tic tac toe, this program will only make moves for the computer
//and thus should only be used on computer turn.

//Function to organize all the other functions and return only what the user wants
//	Parameters - (root) : current game board
//Returns - (Vec<u32>) : game board with next best move 
pub fn next_move(root: Vec<u32>, turn: u32) -> Vec<u32> {
	let point_values = root_board(&root, turn);
	
	if point_values == [0] {
        return root;
	}
	
	place_move(&root, point_values)
}
//Function that takes the point values of each spot and places next move
//	Parameters - (root) : current game board
//			   - (point_values) : point values of each spot on the game board (indicates worth of move)
//Returns - (Vec<u32>) : game board with the next best move
fn place_move(root: &Vec<u32>, mut point_values: Vec<i32>) -> Vec<u32> {
	let copyf_board = point_values.to_vec();
	let largest = copyf_board.iter().max();

	for i in 0..root.len() {
		if root[i] == 1 || root[i] == 2 {
			point_values.insert(i, -255168);
		}
	}

	for i in 0..point_values.len() {
		if point_values[i] == *largest.unwrap() {
			let copy = &mut root.to_vec();
			copy[i] = 1;
			return copy.to_vec()
		}
	}
	//if this is ever accessed the an error has occurred
	[3,3,3, 3,3,3, 3,3,3].to_vec()
}	
//Function that takes the game board and calculates the point value of possible move
//	Parameters - (root) : current game board
//Returns - (Vec<i32>) : point value of each possible move (point values indicates worth of move)
fn root_board(root: &Vec<u32>, mut turn: u32) -> Vec<i32> {
	let end_state = win_loss(root, COMPUTER);
	if end_state == -1 || end_state == 0 || end_state == 1 {
		return [0].to_vec()
	}
	//calculate children of root node
	let children = all_moves(root, turn);

	turn = (turn % 2) + 1;
	
	let mut point_values = Vec::new();

	for i in children.iter() {
		let result = node_solve(i.to_vec(), turn, COMPUTER);
		point_values.push(result);
	}

	println!("{:?}", point_values);

	return point_values;

	/*
	let p_children = all_moves(root, 2);

	for i in p_children.iter() {
		let end_state = win_loss(i);
		loss_values.push(end_state);
	}
	for i in 0..loss_values.len() {
		if loss_values[i] == -1 && point_values[i] != 1 {
			point_values[i] = 255169;
		}
	}

	point_values
	*/
}

/*        -1      <--- Computer Turn
	      /\
	    /   \
	  /      \
	 -1      -1   <--- Player Turn 
	/|\      |
   / | \     |
  +1 0 -1   -1   <--- Outcome 
*/
//This is the basic principle of a min max algorithm
//1 - We must analyze all possible moves and assign a value to the outcomes of the sequences of moves

//2 - We are calculating the best move for the computer, so we assume that the player is always playing the most optimal moves
//    this means that player will always take the branches that minimize the computers chances of winning
//
//    In this case, the second branch has 3 outcomes that player can choose, one outcome leads to a loss so the player will always pick that
//    move, the player will do this for all moves on their turn

//3 - The computer has two possible moves he can make in this tree. both choices lead to a sequence that ends in -1, so the we assign a -1
//    to this tree as that is maximized value that the computer can make

//4 - we go up through the complete game tree assigining values through this method and at the end we can choose the best move with the highest value
//    this move is garunteed to never fail.

//-----------------------------------------------------
// algorithm
// 1 - traverse tree
// 2 - continue down tree until output is reached
// 3 - once output is reached the search must backtrack to reach another part of the tree, compare value to immediate preceding node
//     if node is player node, check whether found value is smaller than current value, if so switch values and continue search
//
//     if node is computer node, check whether found value is larger than current value, if so switch values and continue search
// 4 - when 

//calculates the result of the given node assuming optimal play from both players, should eliminate any chance of losing from the bot

//Function that takes a node and calculates the point values of it, 1 
//meaning the node results in an instant win, -1 indicating instant loss and used to predict the next block
//	Parameters - (node) : one of the children of the current game board
//			   - (turn) : current turn
//Returns - (i32) : a point value of that move (1 means instant win)
fn node_solve(node: Vec<u32>, mut turn: u32, evaluate_player: u32) -> i32 {
	let mut current_value: i32 = UNASSIGNED; //not sure if rust is okay with type casting between i32 and u32...

    let end_game = win_loss(&node, evaluate_player);
    if end_game != 3 {
		return end_game;
	}

    let mut sum = 0;

    let childs = all_moves(&node, turn);

    for child in childs.iter() {
        let found_value = node_solve(child.to_vec(), (turn % 2) + 1, evaluate_player);

		if (current_value == UNASSIGNED) {
			current_value = found_value;
		}
		else if (turn == 1 && found_value > current_value) {
			current_value = found_value;
		}
		else if (turn == 2 && found_value < current_value) {
			current_value = found_value
		}

    }

    current_value
}
//Calculates the state of the board (win/loss/tie)
//	Parameters - (board) : game board
//Returns - (i32) : point values indicating state of board (1 - win, -1 - loss, 0 - tie, 3 - stil going)
pub fn win_loss(board: &Vec<u32>, turn: u32) -> i32 {
	//splitting board
	let mut comp_board = vec![];
	let mut player_board = vec![];

	for i in board {
		if i == &turn {
			comp_board.push(turn);
			player_board.push(0);
		}
		else if i == &((turn%2) + 1) {
			player_board.push((turn % 2) + 1);
			comp_board.push(0);
		}
		else {
			comp_board.push(0);
			player_board.push(0);
		}
	}

	//check if computer won
	let mut result = check_board(&comp_board, turn);
	if result {
		return 1;
	}
    //check if player won
    result = check_board(&player_board, (turn % 2) + 1);
    if result {
        return -1;
    }
    //check if tie
    let mut tie = true;
    for i in board.iter() {
        if i == &0 {
            tie = false;
        }
    }
    if tie == true {
        return 0
    }
    //else game is not over and return as such
    return 3

}

//function checks whether given player has won or not, nothing else
// - &Vec<u32> : board to interpret
// - u32 : player that we're checking
//[bool] - true if player won false otherwise
fn check_board(board: &Vec<u32>, player: u32) -> bool {
	let win_conditions = vec![[player,0,0, player,0,0, player,0,0],
								[0,player,0, 0,player,0, 0,player,0],
								[0,0,player, 0,0,player, 0,0,player],
								[player,player,player, 0,0,0, 0,0,0],
								[0,0,0, player,player,player, 0,0,0],
								[0,0,0, 0,0,0, player,player,player],
								[player,0,0, 0,player,0, 0,0,player],
								[0,0,player, 0,player,0, player,0,0]];

	let mut won = false;
    for i in win_conditions.iter() {
		//println!("{:?} : {:?}", i.to_vec(), board);
		if check_match(board, &i.to_vec(), player) {
			//the board is matched so this player has won
			won = true;
		}
	}

	return won;
}

//check if two boards match
fn check_match(v1: &Vec<u32>, v2: &Vec<u32>, check_value: u32) -> bool  {	
	//do preliminary check
	if (v1.len() != v2.len()) {
		return false;
	}

	for i in 0..v1.len() {
		if v2[i] == check_value && v1[i] != v2[i] {
			return false;
		}
	}

	return true;
}

/*
//function used by win loss to calculate win
fn check_board(board: &Vec<u32>, player: u32) -> i32 {
	let win_conditions = vec![[player,0,0, player,0,0, player,0,0],
								[0,player,0, 0,player,0, 0,player,0],
								[0,0,player, 0,0,player, 0,0,player],
								[player,player,player, 0,0,0, 0,0,0],
								[0,0,0, player,player,player, 0,0,0],
								[0,0,0, 0,0,0, player,player,player],
								[player,0,0, 0,player,0, 0,0,player],
								[0,0,player, 0,player,0, player,0,0]];
	
	let copy_conditions = vec![[player,0,0, player,0,0, player,0,0],
								[0,player,0, 0,player,0, 0,player,0],
								[0,0,player, 0,0,player, 0,0,player],
								[player,player,player, 0,0,0, 0,0,0],
								[0,0,0, player,player,player, 0,0,0],
								[0,0,0, 0,0,0, player,player,player],
								[player,0,0, 0,player,0, 0,0,player],
								[0,0,player, 0,player,0, player,0,0]];

	let mut losses = vec![];
	
	

	let mut win_index = 0;
	let mut remove_index = 0;
	for i in board.iter() {
		for j in win_conditions.iter() {
			if i != &player && j[win_index] == player {
				let mut already_in = false;
				let mut check = 0;
				while losses.len() > check {
					if losses[check] == copy_conditions[remove_index] {
						already_in = true;
					}
					check += 1;
				}
				if already_in == false {
					losses.push(copy_conditions[remove_index]);
				}
			}  
			remove_index += 1;
		}
		remove_index = 0;
		win_index += 1;
	}
	let mut result = 0;
	if win_conditions.len() > losses.len()  {
		if player == 1 {
			result = 1;
		}
		else {
			result = -1;
		}
	}


	result
}
*/
//Function that gets all children of node
//	Parameters - (board) : game board
//	   		   - (turn) : current turn
//Returns - (Vec<Vec<u32>>) : list of all children nodes of given board
fn all_moves(board: &Vec<u32>, turn: u32) -> Vec<Vec<u32>> {
	let mut current_moves = vec![];
	//iterate through the given board
	for i in 0..board.len() {
		if board[i] == 0 {
			let mut current_board = board.to_vec();
			current_board[i] = turn;
			current_moves.push(current_board);
		}
	}

	current_moves
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_win_loss() {
        assert_eq!(win_loss(&vec![1, 1, 1, 0, 0, 0, 0, 0, 0], 1), 1);
		assert_eq!(win_loss(&vec![1, 0, 0, 1, 0, 0, 1, 0, 0], 1), 1);
		assert_eq!(win_loss(&vec![1, 0, 0, 0, 1, 0, 0, 0, 1], 1), 1);

        assert_eq!(win_loss(&vec![2, 2, 2, 0, 0, 0, 0, 0, 0], 2), 1);
		assert_eq!(win_loss(&vec![2, 0, 0, 2, 0, 0, 2, 0, 0], 2), 1);
		assert_eq!(win_loss(&vec![2, 0, 0, 0, 2, 0, 0, 0, 2], 2), 1);
		assert_eq!(win_loss(&vec![1, 2, 2, 1, 2, 1, 2, 1, 0], 2), 1);
    }

	#[test]
	fn test_node_solve() {
		assert_eq!(node_solve(vec![2, 0, 2, 0, 1, 0, 1, 0, 0], 1, 1), -1)
	}
}
