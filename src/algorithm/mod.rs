use crate::map::{grid::Grid, TitleCoords};
pub mod bfs;
pub mod dijkstra;

const ONE_ITERATION_TIME_SEC: f64 = 0.1;

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
    fn name(&self) -> String;
}

pub struct SimulationCoordinator {
    is_processing: bool,
    steps: u32,
    accumulated_time: f64,
    has_completed: bool,
}

impl Default for SimulationCoordinator {
    fn default() -> Self {
        Self {
            is_processing: false,
            steps: 0,
            accumulated_time: 0.0,
            has_completed: false,
        }
    }
}

impl SimulationCoordinator {
    pub fn is_ready_to_execute(&mut self, delta_time: f64) -> bool {
        if self.is_processing || self.should_iterate(delta_time) {
            return true;
        }
        return false;
    }
    pub fn increase_step_count(&mut self) {
        self.steps += 1;
    }

    pub fn is_goal_reached(&mut self, current_title: TitleCoords, goal_title: TitleCoords) -> bool {
        if current_title != goal_title {
            return false;
        }
        self.is_processing = false;
        true
    }

    fn should_iterate(&mut self, delta_time: f64) -> bool {
        self.accumulated_time += delta_time;
        if self.accumulated_time < ONE_ITERATION_TIME_SEC {
            return false;
        }
        self.accumulated_time = 0.0;
        true
    }
}
