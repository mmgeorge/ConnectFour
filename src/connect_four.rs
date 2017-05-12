use std::fmt;

const player_1: usize = 0;
const player_2: usize = 1;

pub struct connect_four {
	board: Vec<Vec<usize>>,
	player_1_turn: bool,
}

impl connect_four {
	pub fn new() -> Self {
		let mut new_board = Vec::new();
		for _x in 0..7 {
			new_board.push(Vec::new());
		}
		connect_four {
			board: new_board,
			player_1_turn: true,
		}
	}

	pub fn insert(&mut self, col: usize) -> Result<(), &'static str> {
		// if isValidMove(col) {
		if col <= 6 {
			if self.player_1_turn {
				self.board[col].push(player_1);
			} else {
				self.board[col].push(player_2);
			}
			self.change_turn();
			Ok(())
		} else {
			Err("invalid move")
		}
	}

	fn change_turn(&mut self) {
		self.player_1_turn = !self.player_1_turn;
	}
	//is game done - Jeanette
	//insert - Kevin
	//switch player (concurrency) - Kevin
	//isValidMove (check if col is full or col is between 0-7 exclusive) - David
}


//David
// impl fmt::Debug for connect_four {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		for _x in 0..7 {
// 			write!(f, "{}", self.board);
// 		}
//     }
// }