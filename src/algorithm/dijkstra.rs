use super::{Algorithm, AlgorithmError, Measurable, Pathfinder, SimulationCoordinator};
use crate::{
    algorithm::get_statistics,
    map::{grid::Grid, TitleCoords},
};
use priority_queue::DoublePriorityQueue;
use std::collections::HashMap;
/// # Dijkstra's Algorithm **(Uniform Cost Search)**
///  Tracks movement costs to reach goal.
#[derive(Default)]
pub struct Dijkstra {
    priority_titles: DoublePriorityQueue<TitleCoords, i32>,
    cost_so_far: HashMap<TitleCoords, i32>,
    sim_coordinator: SimulationCoordinator,
    path_finder: Pathfinder,
}

impl Measurable for Dijkstra {
    fn output_statistics(&self) -> String {
        get_statistics(
            self.name().as_str(),
            self.path_finder.get_path().len(),
            self.sim_coordinator.steps,
            self.cost_so_far.len(),
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
            let goal = grid.goal_title.unwrap();
            let start = grid.start_title.unwrap();
            let current = current_title.0;
            let _priority = current_title.1;

            // Early exit
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
                    let priority = new_cost;
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
        *self = Dijkstra::default();
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
        "Dijkstra".to_string()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use crate::algorithm::ONE_ITERATION_TIME_SEC;
    #[test]
    fn dijkstra_start() {
        let mut dijkstra = Dijkstra::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);

        assert!(dijkstra.start(&mut grid).is_err());

        let start = TitleCoords { x: 0, y: 0 };

        grid.start_title = Some(start);
        grid.goal_title = Some(TitleCoords { x: 1, y: 1 });

        assert!(dijkstra.start(&mut grid).is_ok());
        assert_eq!(*dijkstra.cost_so_far.get(&start).unwrap(), 0);
        assert_eq!(dijkstra.priority_titles.pop_min().unwrap(), (start, 0));
        assert_eq!(*dijkstra.path_finder.path.get(&start).unwrap(), None);
    }

    #[test]
    fn dijkstra_one_step_of_execution() {
        let mut dijkstra = Dijkstra::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);

        grid.start_title = Some(TitleCoords { x: 5, y: 5 });
        grid.goal_title = Some(TitleCoords { x: 8, y: 8 });

        let _ = dijkstra.start(&mut grid);
        dijkstra.execute_step(&mut grid, ONE_ITERATION_TIME_SEC);

        assert_eq!(dijkstra.cost_so_far.len(), 5);

        let cost = dijkstra.cost_so_far.iter().last().unwrap();
        assert_eq!(*cost.1, 1);

        let priority = dijkstra.priority_titles.pop_min().unwrap().1;
        assert_eq!(priority, 1);
    }
}
