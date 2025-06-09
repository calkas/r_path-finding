use super::{Algorithm, AlgorithmError, Measurable, Pathfinder, SimulationCoordinator};
use crate::{
    algorithm::get_statistics,
    map::{grid::Grid, TitleCoords},
};
use priority_queue::DoublePriorityQueue;
use std::collections::HashMap;

/// # A* Algorithm
/// Mix of Dijkstra and Greedy Best First Search
#[derive(Default)]
pub struct AStar {
    priority_titles: DoublePriorityQueue<TitleCoords, i32>,
    cost_so_far: HashMap<TitleCoords, i32>,
    sim_coordinator: SimulationCoordinator,
    path_finder: Pathfinder,
}

impl Measurable for AStar {
    fn output_statistics(&self) -> String {
        get_statistics(
            self.name().as_str(),
            self.path_finder.get_path().len(),
            self.sim_coordinator.steps,
            self.cost_so_far.len(),
        )
    }
}

impl Algorithm for AStar {
    /// # start
    /// A* Algorithm starts.
    ///
    /// Init the algorithm values
    fn start(&mut self, grid: &mut Grid) -> Result<(), super::AlgorithmError> {
        if grid.start_title.is_none() || grid.goal_title.is_none() {
            return Err(AlgorithmError::InvalidInputData);
        }
        let start = grid.start_title.unwrap();

        self.priority_titles.push(start, 0);
        self.cost_so_far.insert(start, 0);
        self.path_finder.add_to_path(start, None);
        self.sim_coordinator.start_processing();

        Ok(())
    }

    /// # execute_step
    /// Algorithm processing update every ONE_ITERATION_TIME_SEC until reach the goal
    fn execute_step(&mut self, grid: &mut Grid, delta_time: f64) {
        if !self.sim_coordinator.is_ready_to_execute(delta_time) {
            return;
        }

        if let Some(current_title) = self.priority_titles.pop_min() {
            self.sim_coordinator.increase_step_count();

            let start = grid.start_title.unwrap();
            let goal = grid.goal_title.unwrap();
            let current = current_title.0;
            let _priority = current_title.1;

            //Early exit
            if self.sim_coordinator.process_goal_reached(current, goal) {
                self.path_finder.reconstruct_path(start, goal);
                for element in self.path_finder.get_path().iter() {
                    grid.set_trace_back_path(*element);
                }
                return;
            }

            grid.mark_visited(current);

            let neighbors = grid.get_neighbors(current);

            for neighbor in neighbors {
                let new_cost =
                    *self.cost_so_far.get(&current).unwrap() + grid.cost(current, neighbor);

                if !self.cost_so_far.contains_key(&neighbor)
                    || new_cost < *self.cost_so_far.get(&neighbor).unwrap()
                {
                    grid.mark_process(neighbor);
                    self.cost_so_far.insert(neighbor, new_cost);
                    let priority = new_cost + grid.heuristic(current, neighbor);
                    self.priority_titles.push(neighbor, priority);
                    self.path_finder.add_to_path(neighbor, Some(current));
                }
            }
            // Check if goal is unreachable
            if self.priority_titles.is_empty() {
                self.sim_coordinator.has_completed = true;
                self.sim_coordinator.stop_processing();
            }
        } else {
            self.sim_coordinator.stop_processing();
        }
    }

    /// # reset
    /// Reset the algorithm processing
    fn reset(&mut self, grid: &mut Grid) {
        *self = AStar::default();
        grid.reset();
    }

    /// # has_completed
    /// Check if processing is done
    fn has_completed(&self) -> bool {
        self.sim_coordinator.has_completed
    }

    /// # name
    /// Algorithm name
    fn name(&self) -> String {
        "A*".to_string()
    }
}
