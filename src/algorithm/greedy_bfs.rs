use super::{
    Algorithm, AlgorithmError, Measurable, Pathfinder, SimulationCoordinator,
    ONE_ITERATION_TIME_SEC,
};
use crate::map::{grid::Grid, TitleCoords};
use priority_queue::DoublePriorityQueue;

// # Greedy Best First Search
#[derive(Default)]
pub struct GreedyBfs {
    priority_titles: DoublePriorityQueue<TitleCoords, i32>,
    visited_titles: Vec<TitleCoords>,
    sim_coordinator: SimulationCoordinator,
    path_finder: Pathfinder,
}

impl Measurable for GreedyBfs {
    fn output_statistics(&self) -> String {
        if self.path_finder.get_path().is_empty() {
            return format!("Goal is unreachable !");
        }
        format!(
            "Greedy BFS Statistics:\n\n - Path length: {}\n - Steps taken: {}\n - Visited nodes: {}\n - Time per iteration: {:.2} sec\n - Total time: {:.2} sec",
            self.path_finder.get_path().len(),
            self.sim_coordinator.steps,
            self.visited_titles.len(),
            ONE_ITERATION_TIME_SEC,
            self.sim_coordinator.steps as f64 * ONE_ITERATION_TIME_SEC
        )
    }
}

impl Algorithm for GreedyBfs {
    fn start(&mut self, grid: &mut Grid) -> Result<(), AlgorithmError> {
        if grid.start_title.is_none() || grid.goal_title.is_none() {
            return Err(AlgorithmError::InvalidInputData);
        }

        let start = grid.start_title.unwrap();
        self.priority_titles.push(start, 0);
        self.path_finder.add_to_path(start, None);
        self.visited_titles.push(start);
        self.sim_coordinator.start_processing();

        Ok(())
    }

    fn execute_step(&mut self, grid: &mut Grid, delta_time: f64) {
        if !self.sim_coordinator.is_ready_to_execute(delta_time) {
            return;
        }

        if let Some(current_title) = self.priority_titles.pop_min() {
            self.sim_coordinator.increase_step_count();
            let current = current_title.0;
            let _priority = current_title.1;
            let goal = grid.goal_title.unwrap();
            let start = grid.start_title.unwrap();

            if self.sim_coordinator.is_goal_reached(current, goal) {
                self.sim_coordinator.has_completed = true;
                self.sim_coordinator.stop_processing();
                self.path_finder.reconstruct_path(start, goal);

                for element in self.path_finder.get_path().iter() {
                    grid.set_trace_back_path(*element);
                }
                return;
            }

            grid.mark_visited(current);
            let neighbors = grid.get_neighbors(current);

            for neighbor in neighbors {
                if !self.visited_titles.contains(&neighbor) {
                    let heuristic_priority = grid.heuristic(neighbor, goal);
                    self.priority_titles.push(neighbor, heuristic_priority);
                    self.visited_titles.push(neighbor);
                    grid.mark_process(neighbor);
                    self.path_finder.add_to_path(neighbor, Some(current));
                }
            }
            // Check if goal is unreachable
            if self.priority_titles.is_empty() {
                self.sim_coordinator.has_completed = true;
                self.sim_coordinator.stop_processing();
                return;
            }
        } else {
            self.sim_coordinator.stop_processing();
        }
    }

    fn reset(&mut self, grid: &mut Grid) {
        *self = GreedyBfs::default();
        grid.reset();
    }

    fn has_completed(&self) -> bool {
        self.sim_coordinator.has_completed
    }

    fn name(&self) -> String {
        format!("Greedy Best First Search")
    }
}
