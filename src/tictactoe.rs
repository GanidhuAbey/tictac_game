//Currently the loss is valued far to low for the computer such that when it sees a move that will inevitable lead to a loss but overall
//has more wins then the other moves then it will simply choose that move and end up losing.

//To fix this I need to reprogram the game such that computer picks the moves with the least losses rather than just having all of them
//cancel out into one move.

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
	println!("{:?}", point_values);
	
	for i in 0..point_values.len() {
		if point_values[i] == 255168 {
			let copy = &mut root.to_vec();
			copy[i] = 1;
			return copy.to_vec()
		}
	}

	for i in 0..point_values.len() {
		if point_values[i] == 255169 {
			let copy = &mut root.to_vec();
			copy[i] = 1;
			return copy.to_vec()
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
	let end_state = win_loss(root);
	if end_state == -1 || end_state == 0 || end_state == 1 {
		return [0].to_vec()
	}
	//calculate children of root node
	let children = all_moves(root, turn);

	if turn == 1 {
		turn = 2;
	}
	else {
		turn = 1
	}
	
	let mut point_values = Vec::new();
	let mut loss_values = Vec::new();

	for i in children.iter() {
		println!("{:?}", i);
		let result = node_solve(i.to_vec(), turn);
		println!("{}", result);
		point_values.push(result);
	}
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
}
//Function that takes a node and calculates the point values of it, 1 
//meaning the node results in an instant win, -1 indicating instant loss and used to predict the next block
//	Parameters - (node) : one of the children of the current game board
//			   - (turn) : current turn
//Returns - (i32) : a point value of that move (1 means instant win)
fn node_solve(node: Vec<u32>, mut turn: u32) -> i32 {
    let end_game = win_loss(&node);
    if end_game == 1 {
        return 255168
    }
    else if end_game == 0 {
        return end_game
    }
    else if end_game == -1 {
        return end_game
    }

    let mut sum = 0;

    let childs = all_moves(&node, turn);

	if turn == 1 {
    	turn = 2;
    }
    else {
        turn = 1;
    }

    for child in childs.iter() {
        sum += node_solve(child.to_vec(), turn);
    }

    sum
}
//Calculates the state of the board (win/loss/tie)
//	Parameters - (board) : game board
//Returns - (i32) : point values indicating state of board (1 - win, -1 - loss, 0 - tie, 3 - stil going)
pub fn win_loss(board: &Vec<u32>) -> i32 {
	//splitting board
	let mut comp_board = vec![];
	let mut player_board = vec![];

	for i in board {
		if i == &1 {
			comp_board.push(1);
			player_board.push(0);
		}
		else if i == &2 {
			player_board.push(2);
			comp_board.push(0);
		}
		else {
			comp_board.push(0);
			player_board.push(0);
		}
	}

	//check if computer won
	let mut result = check_board(&comp_board, 1);
	if result > 0 {
		return result
	}
    //check if player won
    result = check_board(&player_board, 2);
    if result < 0 {
        return result
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
//function used by win loss to calculate win
fn check_board(board: &Vec<u32>, player: u32) -> i32 {
	//println!("hello");
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
			//println!("{:?}:{:?}, {:?}", i, j[win_index], j);
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
	//println!("losses: {:?}, wins: {}", losses, win_conditions.len());
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
