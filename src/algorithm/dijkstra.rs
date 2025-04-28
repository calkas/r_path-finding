use super::ONE_ITERATION_TIME_SEC;
use super::{Algorithm, AlgorithmError, Measurable, SimulationCoordinator};
use crate::map::grid::Grid;
use crate::map::TitleCoords;
use priority_queue::DoublePriorityQueue;
use std::collections::HashMap;
/// # Dijkstra's Algorithm **(Uniform Cost Search)**
///  Tracks movement costs to reach goal.
pub struct Dijkstra {
    priority_titles: DoublePriorityQueue<TitleCoords, i32>,
    cost_so_far: HashMap<TitleCoords, i32>,
    title_path_mapping: HashMap<Option<TitleCoords>, Option<TitleCoords>>,
    path: Vec<TitleCoords>,
    visited_titles: Vec<TitleCoords>,
    sim_coordinator: SimulationCoordinator,
}

impl Default for Dijkstra {
    fn default() -> Self {
        Self {
            priority_titles: DoublePriorityQueue::new(),
            cost_so_far: HashMap::default(),
            title_path_mapping: HashMap::default(),
            path: Vec::new(),
            visited_titles: Vec::new(),
            sim_coordinator: SimulationCoordinator::default(),
        }
    }
}

impl Dijkstra {
    fn build_solution_path(&mut self, grid: &mut Grid) {
        let mut current_title = grid.goal_title;
        while let Some(coord) = current_title {
            self.path.push(coord);
            grid.set_trace_back_path(coord);
            current_title = *self
                .title_path_mapping
                .get(&current_title)
                .expect("Title coord does not exist");
        }
    }

    fn handle_goal_reached(&mut self, grid: &mut Grid) {
        self.sim_coordinator.has_completed = true;
        self.build_solution_path(grid);
    }
}

impl Measurable for Dijkstra {
    fn output_statistics(&self) -> String {
        format!(
            "Statistics:\n - Path length: {}\n - Steps taken: {}\n - Visited nodes: {}\n - Time per iteration: {:.2} sec\n - Total time: {:.2} sec",
            self.path.len(),
            self.sim_coordinator.steps,
            self.visited_titles.len(),
            ONE_ITERATION_TIME_SEC,
            self.sim_coordinator.steps as f64 * ONE_ITERATION_TIME_SEC
        )
    }
}

impl Algorithm for Dijkstra {
    /// # start
    /// Dijkstra's Algorithm starts.
    ///
    /// Init the algorithm values
    fn start(&mut self, grid: &mut Grid) -> Result<(), super::AlgorithmError> {
        if grid.start_title.is_none() || grid.goal_title.is_none() {
            return Err(AlgorithmError::InvalidInputData);
        }
        let start = grid.start_title.unwrap();

        self.priority_titles.push(start, 0);
        self.cost_so_far.insert(start, 0);
        self.title_path_mapping.insert(Some(start), None);

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
            let goal = grid.goal_title.unwrap();
            let current = current_title.0;
            let _priority = current_title.1;

            //Early exit
            if self.sim_coordinator.is_goal_reached(current, goal) {
                self.handle_goal_reached(grid);
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
                    self.visited_titles.push(neighbor);
                    grid.mark_process(neighbor);
                    self.cost_so_far.insert(neighbor, new_cost);
                    let priority = new_cost;
                    self.priority_titles.push(neighbor, priority);
                    self.title_path_mapping
                        .insert(Some(neighbor), Some(current));
                }
            }
        } else {
            self.sim_coordinator.is_processing = false;
        }
    }

    /// # reset
    /// Reset the algorithm processing
    fn reset(&mut self, grid: &mut Grid) {
        *self = Dijkstra::default();
        grid.reset();
    }

    /// # has_completed
    /// Check if processing is done
    fn has_completed(&self) -> bool {
        return self.sim_coordinator.has_completed;
    }

    /// # name
    /// Algorithm name
    fn name(&self) -> String {
        "Dijkstra".to_string()
    }
}
