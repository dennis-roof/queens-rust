mod position;
pub mod board;

use position::Position;
use board::Board;

pub struct QueensSolver {
    pub state_manager: Vec<Board>
}

impl QueensSolver {
    fn is_solved(&self) -> bool
    {
	if self.state_manager.is_empty() {
	    return false;
	}
	
	let number_of_queen_pieces = self.state_manager.len();
	
	let board_size = match self.state_manager.last() {
	    Some(board) => board.size,
	    None => 0,
	};

	(number_of_queen_pieces == board_size)
    }

    pub fn get_all_solutions(&mut self) -> Vec<[[usize; 7]; 7]>
    {
	let mut solutions: Vec<[[usize; 7]; 7]> = Vec::new();

	let mut board: Board = match Board::new(7) {
	    Some(board) => board,
	    None => return solutions,
	};
	self.state_manager.push(board);

	let mut new_board: Board;

	let mut wrong_board: Board;

	let mut position: Position;

	loop {
	    board = match self.state_manager.pop() {
		Some(board) => board,
		None => break,
	    };
	    
	    if self.is_solved() {
		for row in board.queens.iter() {
		    for square in row.iter() {
			match square {
			    1 => print!("#"),
			    _ => print!("."),
			}
		    }
		    println!();
		}
		println!("===");
		solutions.push(board.queens);
	    }
	    // if corrections > 1000 { panic!("42"); }

	    position = board.find_valid_move();

	    // println!("{:?}, {}, {}", &position, board.is_valid_move(&position), self.is_solved());
	    // println!("42 A, boards: {}", self.state_manager.len());

	    if ! board.is_valid_move(&position) || self.is_solved() {
		loop {
		    wrong_board = board;
	    
		    board = match self.state_manager.pop() {
			Some(board) => board,
			None => { board = wrong_board; break },
		    };

		    board.invalidate_move(wrong_board.pos_x, wrong_board.pos_y);

		    position = board.find_valid_move();
		    
		    if board.is_valid_move(&position) { break; }
		}
	    }

	    // println!("42 B, boards: {}", self.state_manager.len());

	    if board.is_valid_move(&position) {
		// println!("valid move!");
		// panic!("24");
		new_board = board.clone();

		self.state_manager.push(board);
		
		new_board.add_queen(&position);
		self.state_manager.push(new_board);
	    }
	}

	println!("done, solutions found: {}", solutions.len());

	solutions
    }
}
