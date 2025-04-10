use super::{Algorithm, AlgorithmError, Measurable, SimulationCoordinator};
use crate::map::grid::Grid;
use crate::map::TitleCoords;
use priority_queue::DoublePriorityQueue;
/// # Dijkstra's Algorithm **(Uniform Cost Search)**
///  Tracks movement costs to reach goal.
pub struct Dijkstra {
    priority_titles: DoublePriorityQueue<TitleCoords, u32>,
    sim_coordinator: SimulationCoordinator,
}

impl Default for Dijkstra {
    fn default() -> Self {
        Self {
            priority_titles: DoublePriorityQueue::new(),
            sim_coordinator: SimulationCoordinator::default(),
        }
    }
}

impl Measurable for Dijkstra {
    fn output_statistics(&self) -> String {
        todo!()
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

        self.priority_titles.push(grid.start_title.unwrap(), 0);

        Ok(())
    }

    /// # execute_step
    /// Algorithm processing update every ONE_ITERATION_TIME_SEC until reach the goal
    fn execute_step(&mut self, grid: &mut Grid, delta_time: f64) {
        if !self.sim_coordinator.is_ready_to_execute(delta_time) {
            return;
        }

        if let Some(current_title) = self.priority_titles.peek_min() {
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
}
