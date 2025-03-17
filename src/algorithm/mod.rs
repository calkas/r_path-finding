use crate::map::grid::Grid;
pub mod bfs;
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
