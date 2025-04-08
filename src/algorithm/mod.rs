use crate::map::grid::Grid;
pub mod bfs;

const ONE_ITERATION_TIME_SEC: f64 = 0.1;

/// FYI the coordinate system is
///
/// (0,0)----> x    <br>
/// |               <br>
/// |               <br>
/// V y             <br>
const POSSIBLE_DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Up, Down, Left, Right

#[derive(Debug)]
pub enum AlgorithmError {
    InvalidInputData,
}

pub trait Measurable {
    fn output_statistics(&self) -> String;
}

pub trait Algorithm: Measurable {
    fn start(&mut self, grid: &mut Grid) -> Result<(), AlgorithmError>;
    fn execute_step(&mut self, grid: &mut Grid, delta_time: f64);
    fn reset(&mut self, grid: &mut Grid);
    fn has_completed(&self) -> bool;
}
