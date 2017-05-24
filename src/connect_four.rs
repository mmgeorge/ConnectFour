use std::fmt;
use std::cmp;
use std::str::FromStr;
use std::io::{BufRead,stdin};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
	One,
	Two,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectK {
	width: usize, 
	height: usize,
	k: usize,
	curr_player: Player,
	board: Vec<Vec<Option<Player>>>,
}


impl ConnectK {
	pub fn new(width: usize, height: usize, k: usize, curr_player: Player) -> Self {
		let mut new_board = Vec::new();
		for _i in 0..width {
			let mut new_row = Vec::new();
			for _j in 0..height {
				new_row.push(None);
			}
			new_board.push(new_row);
		}
		ConnectK {
			width: width,
			height: height,
			k: k,
			curr_player: curr_player,
			board: new_board,
		}
	}


	pub fn play_game(&mut self) {
		let mut game_over = false;
		println!("Player 1 goes first!");
		while !game_over {
			println!("Please enter the column number");
			let col_index = read_input();
			let row_index = self.insert(col_index.clone()).unwrap();
			let check_if_win = self.clone().is_winner(col_index.clone(), row_index.clone());
			if let Some(ref winner) = check_if_win {
				game_over = true;
				println!("The winner is {} - ", *winner);
			}
			println!("{}", self);
		}
		println!("GAME OVER!");
		
	}

	// insert
	// input: the column index which the player chooses to put a marker in
	// output: a Result with either the row index in which the marker was placed
	//         or an error
	pub fn insert(&mut self, col_index: usize) -> Result<usize, &'static str> {
		let row_index = self.check_valid_move(col_index);
		match row_index {
			Some(ri) => {
				if self.curr_player == Player::One {
					self.board[col_index][ri] = Some(Player::One);
				} else {
					self.board[col_index][ri] = Some(Player::Two);
				}
				self.change_turn();
				return Ok(ri);
			},
			None => Err("Insert failed"),
		}
		
	}

	// check_valid_move
	// helper function for insert that checks whether the move is valid
	// input: the column index
	// output: an Option of the row index in which the marker is to be placed
	//         or None
	fn check_valid_move(&self, col_index: usize) -> Option<usize> {
		if col_index <= self.width {
			for i in 0..self.height {
				if self.board[col_index][i] == None {
					return Some(i);
				}
			}
		}

		//not a valid move
		None
	}

	// change_turn
	// changes the current player
	fn change_turn(&mut self) {
		if self.curr_player == Player::One {
			self.curr_player = Player::Two;
		} else {
			self.curr_player = Player::One;
		}
	}

	// is_winner
	// checks whether either player is the winner
	// input: position that was just played (col_index, row_index)
	// output: an Option<Player> that returns the winner or None
	pub fn is_winner(self, col_index: usize, row_index: usize) -> Option<Player> {
		//check both players to see if there is a winner
		let check_players = vec![Player::One, Player::Two];

		let mut connected;
		let min_width;
		let min_height;

		//comparisons for usize (otherwise will complain about subtract overflow)
		if col_index >= self.k { min_width = col_index - self.k; } else { min_width = 0; }
		if row_index >= self.k { min_height = row_index - self.k; } else { min_height = 0; }
		let max_width  = cmp::min(self.width, col_index + self.k);
		let max_height = cmp::min(self.height, row_index + self.k);

		//check if either player is the winner
		for player in check_players {

			for col in min_width..max_width {
				connected = 0;
				for row in min_height..max_height {
					if connected == self.k {
						return Some(player);
					}
					if self.board[col][row] == Some(player) {
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
					if connected == self.k {
						return Some(player);
					}
					if self.board[col][row] == Some(player) {
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
					if connected == self.k {
						return Some(player);
					}
					if row + diag < self.height {
						if self.board[diag][row+diag] == Some(player) {
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
					if connected == self.k {
						return Some(player);
					}
					if row >= diag && (diag == 0 && row < self.height) {
						if self.board[diag][row-diag] == Some(player) {
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
					if connected == self.k {
						return Some(player);
					}
					if col + diag < self.width {
						if self.board[col+diag][diag] == Some(player) {
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
					if connected == self.k {
						return Some(player);
					}
					if col >= diag && (diag == 0 && col < self.width) {
						if self.board[col-diag][diag] == Some(player) {
							connected += 1;
						} else {
							connected = 0;
						}
					} else {
						break;
					}
				}
			}

		}

		None
	}

	

	pub fn printer(&self) {
		println!("{}", self);
		if self.curr_player == Player::One {
    		println!("{}", "Player 1's Turn");
    	} else {
    		println!("{}", "Player 2's Turn");
    	}

	}

}


pub fn read_input() -> usize {
	let stdin = stdin();
	let line = stdin.lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");
    usize::from_str(&line).unwrap()
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Player::One => "Player One",
            Player::Two => "Player Two",
        };
        write!(f, "{}\n", printable)
    }
}



impl fmt::Display for ConnectK {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut ret_str = String::from("\n");
		let mut row_offset = self.height-1;

		for _row in 0..self.height {
			ret_str.push_str("|");
			for col in 0..self.width-1 {
				let index = &self.board[col][row_offset]; // correlates to x,y coord 
				if *index == Some(Player::One) { ret_str.push_str(" x |") }
				else if *index == Some(Player::Two)  { ret_str.push_str(" o |") }
				else { ret_str.push_str("   |") }
				
			}
			if row_offset > 0 { row_offset -=  1; }
			ret_str.push_str("\n");
		}
		ret_str.push_str("| - | - | - | - | - | - |"); // add the bottom of the board
		write!(f, "{}", ret_str)
    }
}




#[cfg(test)]
mod is_winner_test {
	use super::{ConnectK, Player};

	#[test]
	fn no_winner() {
		let board = ConnectK::new(7,6,4,Player::One);
		assert_eq!(None, board.is_winner(0, 0));
	}

	#[test]
	fn player_1_winner() {
		let mut board = ConnectK::new(7,6,4,Player::One);
		board.insert(1).unwrap(); //player 1
		board.insert(0).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		board.insert(0).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		board.insert(0).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		//player one wins
		assert_eq!(Player::One, board.clone().is_winner(1, 3).unwrap());
	}

	#[test]
	fn player_2_winner() {
		let mut board = ConnectK::new(7,6,4,Player::One);
		board.insert(0).unwrap(); //player 1
		board.insert(1).unwrap(); //player 2
		board.insert(0).unwrap(); //player 1
		board.insert(2).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		board.insert(3).unwrap(); //player 2
		board.insert(1).unwrap(); //player 1
		board.insert(4).unwrap(); //player 2
		//player two wins
		assert_eq!(Player::Two, board.clone().is_winner(4, 0).unwrap());
	}

	#[test]
	fn player_1_diag_win() {
		let mut board = ConnectK::new(7,6,4,Player::One);
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
		assert_eq!(Player::One, board.clone().is_winner(3, 3).unwrap());

	}
}


mod format_tests {
	use super::{ConnectK,Player};

	// this test is just to see the printed output 
	// if you run : cargo test -- --nocapture 
	// it displays the standard output
	
	#[test]
    fn display_empty_board() {
    	let mut board = ConnectK::new(7,6,4,Player::One);
    	board.insert(0).unwrap();
    	board.insert(0).unwrap();
    	board.insert(1).unwrap();
    	println!("{}", board);
    	if board.curr_player == Player::One {
    		println!("{}", "Player 1's Turn");
    	} else {
    		println!("{}", "Player 2's Turn");
    	}
    }
}


