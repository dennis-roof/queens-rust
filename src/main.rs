mod queens_solver;

use queens_solver::board::Board;
use queens_solver::QueensSolver;

fn main() {
    let state_manager: Vec<Board> = Vec::new();
    let mut queens_solver = QueensSolver{state_manager};
    let _solutions: Vec<[[usize; 7]; 7]> = queens_solver.get_all_solutions();
}
