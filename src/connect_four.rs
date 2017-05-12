use std::fmt;

const PLAYER_1: usize = 1;
const PLAYER_2: usize = 2;
const WIDTH   : usize = 7;
const HEIGHT  : usize = 6;

pub type Player = usize; //player == 1 or 2

pub struct ConnectFour {
	board: [[usize; HEIGHT]; WIDTH],
	player_1_turn: bool,
	
}

//insert - Kevin
//switch player (concurrency) - Kevin
//isValidMove (check if col is full or col is between 0-7 exclusive) - David

impl ConnectFour {
	pub fn new() -> Self {
		let new_board = [[0; HEIGHT]; WIDTH];
		ConnectFour {
			board: new_board,
			player_1_turn: true,
		}
	}

	pub fn insert(&mut self, col_index: usize) -> Result<(), &'static str> {
		// if isValidMove(col_index) {
		if col_index <= WIDTH {
			let row_index = self.find_col_height(col_index);
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
				if row - diag >= 0 {
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
				if col - diag >= 0 {
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

		return false;
	}
	
}


#[cfg(test)]
mod insert_test {
	use super::ConnectFour;
	#[test] 
	fn column_too_high() {
		
	}

	fn validate_insert() {
		let mut game = ConnectFour::new();
	}
}

//David
// impl fmt::Debug for ConnectFour {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		for i in 0..WIDTH {
// 			write!(f, "{:?}", self.board[i]);
// 		}
		
//     }
// }