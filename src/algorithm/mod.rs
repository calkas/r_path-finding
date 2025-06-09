use crate::map::{grid::Grid, TitleCoords};
use std::collections::HashMap;
pub mod a_star;
pub mod bfs;
pub mod dijkstra;
pub mod greedy_bfs;

const ONE_ITERATION_TIME_SEC: f64 = 0.01;

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
        if self.is_processing && self.should_iterate(delta_time) {
            return true;
        }
        false
    }
    pub fn increase_step_count(&mut self) {
        self.steps += 1;
    }

    pub fn start_processing(&mut self) {
        self.is_processing = true;
    }

    pub fn stop_processing(&mut self) {
        self.is_processing = false;
    }

    pub fn process_goal_reached(&mut self, current: TitleCoords, goal: TitleCoords) -> bool {
        if current == goal {
            self.has_completed = true;
            self.stop_processing();
            return true;
        }
        false
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

fn get_statistics(
    algorithm_name: &str,
    path_length: usize,
    steps: u32,
    num_visited_titles: usize,
) -> String {
    if path_length == 0 {
        return "Goal is unreachable !".to_string();
    }

    format!(
            " {}\n\n Statistics:\n - Path length: {}\n - Steps taken: {}\n - Visited nodes: {}\n - Time per iteration: {:.2} sec\n - Total time: {:.2} sec",
            algorithm_name,
            path_length,
            steps,
            num_visited_titles,
            ONE_ITERATION_TIME_SEC,
            steps as f64 * ONE_ITERATION_TIME_SEC
        )
}

#[derive(Default)]
pub struct Pathfinder {
    path: HashMap<TitleCoords, Option<TitleCoords>>,
    solution_path: Vec<TitleCoords>,
}

impl Pathfinder {
    pub fn add_to_path(&mut self, came_from: TitleCoords, to: Option<TitleCoords>) {
        self.path.insert(came_from, to);
    }
    pub fn reconstruct_path(&mut self, start: TitleCoords, goal: TitleCoords) {
        let mut current = goal;

        while current != start {
            self.solution_path.push(current);
            let c = *self
                .path
                .get(&current)
                .expect("Title for path reconstructing is wrong");
            current = c.unwrap();
        }
        self.solution_path.push(start);
    }
    pub fn get_path(&self) -> &Vec<TitleCoords> {
        &self.solution_path
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn simulation_coordinator() {
        let mut sim = SimulationCoordinator::default();

        // Ready for execution test case
        let delta_time = ONE_ITERATION_TIME_SEC;

        sim.stop_processing();
        assert!(!sim.is_ready_to_execute(delta_time));

        sim.start_processing();
        assert!(sim.is_ready_to_execute(delta_time));

        // Goal Reached test case
        let start = TitleCoords { x: 0, y: 0 };
        let mut goal = TitleCoords { x: 0, y: 1 };
        assert!(!sim.process_goal_reached(start, goal));

        goal = start;
        assert!(sim.process_goal_reached(start, goal));

        // Increase step case
        sim.increase_step_count();
        sim.increase_step_count();
        sim.increase_step_count();

        assert_eq!(sim.steps, 3);
    }
    #[test]
    fn path_finding() {
        let mut path_finder = Pathfinder::default();

        // [start] -> [1] -> [2] -> [3] -> [goal]
        let path_start = TitleCoords { x: 0, y: 0 };
        let path_1 = TitleCoords { x: 1, y: 0 };
        let path_2 = TitleCoords { x: 2, y: 0 };
        let path_3 = TitleCoords { x: 3, y: 0 };
        let path_goal = TitleCoords { x: 3, y: 1 };

        let exp_solution_path = vec![path_goal, path_3, path_2, path_1, path_start];

        path_finder.add_to_path(path_start, None);
        path_finder.add_to_path(path_1, Some(path_start));
        path_finder.add_to_path(path_2, Some(path_1));
        path_finder.add_to_path(path_3, Some(path_2));
        path_finder.add_to_path(path_goal, Some(path_3));

        path_finder.reconstruct_path(path_start, path_goal);

        assert_eq!(exp_solution_path, *path_finder.get_path());
    }
}
