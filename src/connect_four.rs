use std::fmt;

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

	//is game done - Jeanette
	//insert - Kevin
	//switch player (concurrency) - Kevin
	//isValidMove (check if col is full or col is between 0-7 exclusive) - David
}


//David
impl fmt::Debug for connect_four {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for _x in 0..7 {
			write!(f, "{}", self.board);
		}
    }
}