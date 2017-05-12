// use std::fmt;
pub type Player = usize;

const PLAYER_1: Player = 1;
const PLAYER_2: Player = 2;
const WIDTH   : usize = 7;
const HEIGHT  : usize = 6;


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

	// pub fn insert(&mut self, col: usize) -> Result<(), &'static str> {
	// 	// if isValidMove(col) {
	// 	if col <= 6 {
	// 		if self.player_1_turn {
	// 			self.board[col](PLAYER_1);
	// 		} else {
	// 			self.board[col].push(PLAYER_2);
	// 		}
	// 		self.change_turn();
	// 		Ok(())
	// 	} else {
	// 		Err("invalid move")
	// 	}
	// }

	fn change_turn(&mut self) {
		self.player_1_turn = !self.player_1_turn;
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
				println!("hey4");
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

#[cfg(test)]
mod is_winner_test {
	use super::ConnectFour;

	#[test]
	fn no_winner() {
		let board = ConnectFour::new();
		assert_eq!(false, board.is_winner(1));
	}

	// #[test]
	// fn yes_winner() {
	// 	let board = ConnectFour::new();
	// }
}

//David
// impl fmt::Debug for connect_four {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		for _x in 0..7 {
// 			write!(f, "{}", self.board);
// 		}
//     }
// }