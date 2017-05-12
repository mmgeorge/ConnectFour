// use std::fmt;

const WIDTH : usize = 7;
const HEIGHT: usize = 6;

pub type Player = usize; //player == 1 or 2

pub struct ConnectFour {
	board: Vec<Vec<Player>>,
	player_1_turn: bool,
}

//insert - Kevin
//switch player (concurrency) - Kevin
//isValidMove (check if col is full or col is between 0-7 exclusive) - David

impl ConnectFour {
	pub fn new() -> Self {
		let mut new_board = Vec::new();
		let mut new_col;
		for _x in 0..WIDTH {
			new_col = Vec::new();
			for _y in 0..HEIGHT {
				new_col.push(0);
			}
			new_board.push(new_col);
		}
		ConnectFour {
			board: new_board,
			player_1_turn: true,
		}
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


//David
// impl fmt::Debug for connect_four {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		for _x in 0..7 {
// 			write!(f, "{}", self.board);
// 		}
//     }
// }