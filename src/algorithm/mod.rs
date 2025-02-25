use crate::map::grid::Grid;
pub mod bfs;

pub trait Algorithm {
    fn start(&mut self, grid: &mut Grid) -> bool;
    fn update(&mut self, grid: &mut Grid, delta_time: f64);
    fn reset(&mut self, grid: &mut Grid);
}
