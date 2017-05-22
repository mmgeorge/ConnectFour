use std::fmt;
use std::cmp;
use std::str::FromStr;
use std::io::{BufRead,stdin};
use std::collections::HashMap;


pub type Player = usize;

const PLAYER_1: Player = 1;
const PLAYER_2: Player = 2;
const WIDTH   : usize = 7;
const HEIGHT  : usize = 6;

#[derive(Clone)]
pub struct ConnectFour {
	board: [[Player; HEIGHT]; WIDTH],
	player_1_turn: bool,
}

// finish oData library - Matt
// GTK client GUI - Kevin & David

pub fn read_input() -> usize {
	let stdin = stdin();
	let line = stdin.lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");
    usize::from_str(&line).unwrap()
}


impl ConnectFour {
	pub fn new() -> Self {
		let new_board = [[0; HEIGHT]; WIDTH];
		ConnectFour {
			board: new_board,
			player_1_turn: true,
		}
	}


	pub fn play_game(&mut self) {
		let mut game_over = false;
		println!("Player 1 goes first!");
		while !game_over {
			println!("Please enter the column number");
			let col_index = read_input();
			let row_index = self.insert(col_index.clone()).unwrap();
			if self.clone().is_winner(PLAYER_1, col_index.clone(), row_index.clone()) || self.clone().is_winner(PLAYER_2, col_index.clone(), row_index.clone()) {
				game_over = true;
			}
			println!("{}", self);
		}
		println!("GAME OVER!");
	}


	

	pub fn insert(&mut self, col_index: usize) -> Result<usize, &'static str> {
		let row_index = self.check_valid_move(col_index);
		if row_index != 9999 {
			if self.player_1_turn {
				self.board[col_index][row_index] = PLAYER_1;
			} else {
				self.board[col_index][row_index] = PLAYER_2;
			}
			self.change_turn();
			return Ok(row_index);
		} else {
			Err("Invalid move")
		}
	}
	
	fn check_valid_move(&self, col_index: usize) -> usize {
		if col_index <= WIDTH {
			let row_index = self.find_col_height(col_index);
			if row_index <= HEIGHT { return row_index; }
			else { return 9999; }
		}
		9999
	}


	// Returns first empty spot in column
	fn find_col_height(&self, col_index: usize) -> usize {
		let column = self.board[col_index];
		for i in 0..HEIGHT {
			if column[i] == 0 {
				return i;
			}
		}
		HEIGHT + 1
	}
	fn change_turn(&mut self) {
		self.player_1_turn = !self.player_1_turn;
	}

	// fixing is_winner to only check columns that were played
	pub fn is_winner(self, player: Player, col_index: usize, row_index: usize) -> bool {
		let mut connected;
		let min_width;
		let min_height;

		//comparisons for usize (otherwise will complain about subtract overflow)
		if col_index >= 4 { min_width = col_index - 4; } else { min_width = 0; }
		if row_index >= 4 { min_height = row_index - 4; } else { min_height = 0; }

		let max_width  = cmp::min(WIDTH, col_index + 4);
		let max_height = cmp::min(HEIGHT, row_index + 4);

		for col in min_width..max_width {
			connected = 0;
			for row in min_height..max_height {
				if connected == 4 {
					return true;
				}
				if self.board[col][row] == player {
					connected += 1;
				}
				else {
					connected = 0;
				}
			}
		}

		for row in min_height..max_height {
			connected = 0;
			for col in min_width..max_width {
				if connected == 4 {
					return true;
				}
				if self.board[col][row] == player {
					connected += 1;
				}
				else {
					connected = 0;
				}
			}
		}

		//DIAGONALS
		for row in min_width..max_width {
			connected = 0;
			for diag in min_width..max_width {
				if connected == 4 {
					return true;
				}
				if row + diag < HEIGHT {
					if self.board[diag][row+diag] == player {
						connected += 1;
					} else {
						connected = 0;
					}
				} else {
					break;
				}
			}
		}

		for row in min_width..max_width {
			connected = 0;
			for diag in min_width..max_width {
				if connected == 4 {
					return true;
				}
				if row >= diag && (diag == 0 && row < HEIGHT) {
					if self.board[diag][row-diag] == player {
						connected += 1;
					} else {
						connected = 0;
					}
				} else {
					break;
				}
			}
		}
		for col in min_height..max_height {
			connected = 0;
			for diag in min_height..max_height {
				if connected == 4 {
					return true;
				}
				if col + diag < WIDTH {
					if self.board[col+diag][diag] == player {
						connected += 1;
					} else {
						connected = 0;
					}
				} else {
					break;
				}
			}
		}

		for col in min_height..max_height {
			connected = 0;
			for diag in min_height..max_height {
				if connected == 4 {
					return true;
				}
				if col >= diag && (diag == 0 && col < WIDTH) {
					if self.board[col-diag][diag] == player {
						connected += 1;
					} else {
						connected = 0;
					}
				} else {
					break;
				}
			}
		}

		false
	}

	

	pub fn printer(&self) {
		println!("{}", self);
		if self.player_1_turn {
    		println!("{}", "Player 1's Turn");
    	} else {
    		println!("{}", "Player 2's Turn");
    	}

	}
	


	// //// MINIMAX
	// //https://github.com/erikackermann/Connect-Four/blob/master/minimax.py
	// pub fn minimax(&self, depth: &usize) -> isize {
	// 	let legal_moves = HashMap::new();
	// 	let curr_player;
	// 	let opp_player;
	// 	if player_1_turn { curr_player = PLAYER_2; opp_player = PLAYER_1; }
	// 	else { curr_player = PLAYER_1; opp_player = PLAYER_2; }
	// 	for col in 0..WIDTH {
	// 		if self.is_valid_move(col) {
	// 			let temp_board = self.make_move(col, curr_player);
	// 			legal_moves.insert(col, -1*self.search(depth - 1, temp_board, opp_player));
	// 		}
	// 	}

	// 	let mut best_alpha = isize::min_value();
	// 	let mut best_move = -1;

	// 	for (curr_move, alpha) in &legal_moves {
	// 		if alpha > best_alpha {
	// 			best_alpha = alpha;
	// 			best_move = curr_move;
	// 		}
	// 	}

	// 	best_move
	// }

	// pub fn make_move(&self, col: usize, curr_player: Player) -> Vec<Vec<Player>> {
	// 	let mut c = Vec::new();
	// 	let mut r = Vec::new();


	// }

}


// #[cfg(test)]
// mod insert_test {
// 	use super::ConnectFour;
// 	#[test] 
// 	fn column_too_high() {
		
// 	}

// 	fn validate_insert() {
// 		let mut game = ConnectFour::new();
// 	}


// }


#[cfg(test)]
mod is_winner_test {
	use super::ConnectFour;

	#[test]
	fn no_winner() {
		let board = ConnectFour::new();
		assert_eq!(false, board.is_winner(1, 0, 0));
	}

	#[test]
	fn player_1_winner() {
		let mut board = ConnectFour::new();
		board.insert(1).unwrap(); //player 1
		board.insert(0).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		board.insert(0).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		board.insert(0).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		//player one wins
		assert_eq!(true, board.clone().is_winner(1, 1, 3));
		assert_eq!(false, board.clone().is_winner(2, 1, 3));
	}

	#[test]
	fn player_2_winner() {
		let mut board = ConnectFour::new();
		board.insert(0).unwrap(); //player 1
		board.insert(1).unwrap(); //player 2
		board.insert(0).unwrap(); //player 1
		board.insert(2).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		board.insert(3).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		board.insert(4).unwrap(); //player 2
		//player two wins
		assert_eq!(true, board.clone().is_winner(2, 4, 0));
		assert_eq!(false, board.clone().is_winner(1, 4, 0));
	}

	#[test]
	fn player_1_diag_win() {
		let mut board = ConnectFour::new();
		board.insert(0).unwrap(); //player 1
		board.insert(1).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		board.insert(2).unwrap(); //player 2
		board.insert(2).unwrap(); //player 1
		board.insert(3).unwrap(); //player 2
		board.insert(2).unwrap(); //player 1
		board.insert(3).unwrap(); //player 2
		board.insert(3).unwrap(); //player 1
		board.insert(5).unwrap(); //player 2
		board.insert(3).unwrap(); //player 1
		//player 1 wins
		assert_eq!(true, board.clone().is_winner(1, 3, 3));
		assert_eq!(false, board.clone().is_winner(2, 3, 3));

	}
}




impl fmt::Display for ConnectFour {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut ret_str = String::from("\n");
		let mut row_offset = HEIGHT-1;

		for _row in 0..HEIGHT {
			ret_str.push_str("|");
			for col in 0..WIDTH-1 {
				let index = &self.board[col][row_offset]; // correlates to x,y coord 
				if *index == 1 { ret_str.push_str(" x |") }
				else if *index == 2  { ret_str.push_str(" o |") }
				else { ret_str.push_str("   |") }
				
			}
			if row_offset > 0 { row_offset -=  1; }
			ret_str.push_str("\n");
		}
		ret_str.push_str("| - | - | - | - | - | - |"); // add the bottom of the board
		write!(f, "{}", ret_str)
    }
}



mod format_tests {
	use super::ConnectFour;

	// this test is just to see the printed output 
	// if you run : cargo test -- --nocapture 
	// it displays the standard output
	
	#[test]
    fn display_empty_board() {
    	let mut board = ConnectFour::new();
    	board.insert(0).unwrap();
    	board.insert(0).unwrap();
    	board.insert(1).unwrap();
    	println!("{}", board);
    	if board.player_1_turn {
    		println!("{}", "Player 1's Turn");
    	} else {
    		println!("{}", "Player 2's Turn");
    	}
    }
}
