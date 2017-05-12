use std::fmt;

const PLAYER_1: usize = 1;
const PLAYER_2: usize = 2;
const WIDTH   : usize = 7;
const HEIGHT  : usize = 6;


pub struct connect_four {
	board: [[usize; HEIGHT]; WIDTH],
	player_1_turn: bool,
}

impl connect_four {
	pub fn new() -> Self {
		let new_board = [[0; HEIGHT]; WIDTH];
		connect_four {
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
	//insert - Kevin
	//switch player (concurrency) - Kevin
	//isValidMove (check if col is full or col is between 0-7 exclusive) - David
}


#[cfg(test)]
mod insert_test {
	use super::connect_four;
	#[test] 
	fn column_too_high() {
		
	}

	fn validate_insert() {
		let mut game = connect_four::new();
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