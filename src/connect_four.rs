use std::fmt;

pub type Player = usize;

const PLAYER_1: Player = 1;
const PLAYER_2: Player = 2;
const WIDTH   : usize = 7;
const HEIGHT  : usize = 6;

#[derive(Clone)]
pub struct Connect_Four {
	board: [[usize; HEIGHT]; WIDTH],
	player_1_turn: bool,
	
}

//insert - Kevin
//switch player (concurrency) - Kevin
//isValidMove (check if col is full or col is between 0-7 exclusive) - David

impl Connect_Four {
	pub fn new() -> Self {
		let new_board = [[0; HEIGHT]; WIDTH];
		Connect_Four {
			board: new_board,
			player_1_turn: true,
		}
	}

	pub fn insert(&mut self, col_index: usize) -> Result<(), &'static str> {
		// if isValidMove(col_index) {
		if col_index <= WIDTH {
			let row_index = self.find_col_height(col_index);
			println!("{}", row_index);
			// Temp row_index before isValidMove is implemented
			if row_index <= HEIGHT {
				if self.player_1_turn {
					self.board[col_index][row_index] = PLAYER_1;
				} else {
					self.board[col_index][row_index] = PLAYER_2;
				}
				self.change_turn();
				()
			} 
			Err("invalid move, row index invalid")
		} else {
			Err("invalid move, column index invalid")
		}
	}

	fn change_turn(&mut self) {
		self.player_1_turn = !self.player_1_turn;
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

	//is game done - Jeanette
	pub fn is_winner(self, player: Player) -> bool {
		let mut connected;
		for col in 0..WIDTH {
			connected = 0;
			for row in 0..HEIGHT {
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

		for row in 0..HEIGHT {
			connected = 0;
			for col in 0..WIDTH {
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
		for row in 0..WIDTH {
			connected = 0;
			for diag in 0..WIDTH {
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

		for row in 0..WIDTH {
			connected = 0;
			for diag in 0..WIDTH {
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
		for col in 0..HEIGHT {
			connected = 0;
			for diag in 0..HEIGHT {
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

		for col in 0..HEIGHT {
			connected = 0;
			for diag in 0..HEIGHT {
				if connected == 4{
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

	// isValidMove (check if col is full or col is between 0-7 exclusive) - David
	// Not a public function -- used by insert to indicate if it is a valid move or not
	// insert handles the response to the user 
	fn is_valid_move(&self, column: usize, index: usize) -> bool {

		// Case 1: move is not on the board 
		if column > WIDTH || index > HEIGHT || column < 0 || index < 0 {
			return false;
		}

		// Case 2: check if column is full or index is occupied 
		if self.board[column][HEIGHT-1] != 0  || self.board[column][index] != 0 {
			return false;
		}

		true
	}

	pub fn printer(&self) {
		println!("{}", self);
		if self.player_1_turn {
    		println!("{}", "Player 1's Turn");
    	} else {
    		println!("{}", "Player 2's Turn");
    	}

	}
	
}


#[cfg(test)]
mod insert_test {
	use super::Connect_Four;
	#[test] 
	fn column_too_high() {
		
	}

	fn validate_insert() {
		let mut game = Connect_Four::new();
	}


}


#[cfg(test)]
mod is_winner_test {
	use super::Connect_Four;

	#[test]
	fn no_winner() {
		let board = Connect_Four::new();
		assert_eq!(false, board.is_winner(1));
	}

	#[test]
	fn player_1_winner() {
		let mut board = Connect_Four::new();
		board.insert(1); //player 1
		board.insert(0); //player 2
		board.insert(1); //player 1
		board.insert(0); //player 2
		board.insert(1); //player 1
		board.insert(0); //player 2
		board.insert(1); //player 1
		//player one wins
		assert_eq!(true, board.clone().is_winner(1));
		assert_eq!(false, board.clone().is_winner(2));
	}

	#[test]
	fn player_2_winner() {
		let mut board = Connect_Four::new();
		board.insert(0); //player 1
		board.insert(1); //player 2
		board.insert(0); //player 1
		board.insert(2); //player 2
		board.insert(1); //player 1
		board.insert(3); //player 2
		board.insert(1); //player 1
		board.insert(4); //player 2
		//player two wins
		assert_eq!(true, board.clone().is_winner(2));
		assert_eq!(false, board.clone().is_winner(1));
	}

	#[test]
	fn player_1_diag_win() {
		let mut board = Connect_Four::new();
		board.insert(0); //player 1
		board.insert(1); //player 2
		board.insert(1); //player 1
		board.insert(2); //player 2
		board.insert(2); //player 1
		board.insert(3); //player 2
		board.insert(2); //player 1
		board.insert(3); //player 2
		board.insert(3); //player 1
		board.insert(5); //player 2
		board.insert(3); //player 1
		//player 1 wins
		assert_eq!(true, board.clone().is_winner(1));
		assert_eq!(false, board.clone().is_winner(2));

	}
}




impl fmt::Display for Connect_Four {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut ret_str = String::from("\n");
		let mut row_offset = HEIGHT-1;

		for row in 0..HEIGHT {
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
	use super::*;

	// this test is just to see the printed output 
	// if you run : cargo test -- --nocapture 
	// it displays the standard output
	
	#[test]
    fn display_empty_board() {
    	let mut board = Connect_Four::new();
    	let res = board.insert(0);
    	let res_2 = board.insert(0);
    	let res_3 = board.insert(1);
    	println!("{}", board);
    	if board.player_1_turn {
    		println!("{}", "Player 1's Turn");
    	} else {
    		println!("{}", "Player 2's Turn");
    	}
    }
}
