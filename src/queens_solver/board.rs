use crate::queens_solver::position::Position;

#[derive(Clone, Debug)]
pub struct Board {
    pub pos_x: usize,
    pub pos_y: usize,
    pub size: usize,
    pub queens: [[usize; 7]; 7],
    pub valid_moves: [[usize; 7]; 7],
}

impl Board {
    pub fn new(size: usize) -> Option<Board> {
	match size {
	    size if size > 0 =>
		Some( Board {
		    pos_x: 0,
		    pos_y: 0,
		    size,
		    queens: Board::generate_board(size, 0),
		    valid_moves: Board::generate_board(size, 1),
		} ),
	    _ => None
		
	}	
    }
}

impl Board {
    fn generate_board(_board_size: usize, filler: usize) -> [[usize; 7]; 7]
    {
	let board: [[usize; 7]; 7] = [[filler; 7]; 7];
	board
    }

    pub fn invalidate_move(&mut self, x: usize, y: usize)
    {
	self.valid_moves[y][x] = 0;
    }

    pub fn find_valid_move(&self) -> Position
    {
	for y in 0..self.valid_moves.len() {
	    for x in 0..self.valid_moves[y].len() {
		if self.valid_moves[y][x] == 1 {
		    return Position {x, y};
		}
	    }
	}

	(Position {x: 99, y: 99})
    }

    fn is_diagonal(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool
    {
	let x_diff: i32 = x1 as i32 - x2 as i32;
	let y_diff: i32 = y1 as i32 - y2 as i32;
	(x_diff.abs() == y_diff.abs())
    }

    pub fn is_valid_move(&self, position: &Position) -> bool
    {
	(position.x != 99 && position.y != 99)
    }

    pub fn add_queen(&mut self, pos: &Position)
    {
	self.pos_x = pos.x;
	self.pos_y = pos.y;

	self.queens[pos.x][pos.y] = 1;

	for y in 0..self.valid_moves.len() {
	    for x in 0..self.valid_moves[y].len() {
		if self.valid_moves[y][x] == 0 {
		    continue;
		}

		if x==pos.x || y==pos.y || self.is_diagonal(pos.x, pos.y, x, y) {
		    self.valid_moves[y][x] = 0;
		}
	    }
	}
    }
}
