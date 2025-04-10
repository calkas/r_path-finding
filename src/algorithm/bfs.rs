use super::ONE_ITERATION_TIME_SEC;
use super::{Algorithm, AlgorithmError, Measurable, SimulationCoordinator};
use crate::map::grid::Grid;
use crate::map::TitleCoords;
use std::collections::{HashMap, VecDeque};

/// # Breadth-First Search Algorithm
pub struct Bfs {
    title_processing_queue: VecDeque<TitleCoords>,
    visited_titles: Vec<TitleCoords>,
    path: Vec<TitleCoords>,
    title_path_mapping: HashMap<Option<TitleCoords>, Option<TitleCoords>>,
    sim_coordinator: SimulationCoordinator,
}

impl Default for Bfs {
    fn default() -> Self {
        Self {
            title_processing_queue: VecDeque::default(),
            visited_titles: Vec::default(),
            path: Vec::default(),
            title_path_mapping: HashMap::default(),
            sim_coordinator: SimulationCoordinator::default(),
        }
    }
}

impl Bfs {
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

impl Measurable for Bfs {
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

impl Algorithm for Bfs {
    /// # start
    /// BFS Algorithm starts.
    ///
    /// Init the algorithm values
    fn start(&mut self, grid: &mut Grid) -> Result<(), AlgorithmError> {
        if grid.start_title.is_none() || grid.goal_title.is_none() {
            return Err(AlgorithmError::InvalidInputData);
        }
        let start = grid.start_title.unwrap();

        self.title_processing_queue.push_back(start);
        self.visited_titles.push(start);
        self.title_path_mapping.insert(Some(start), None);
        self.sim_coordinator.is_processing = true;
        Ok(())
    }

    /// # execute_step
    /// Algorithm processing update every ONE_ITERATION_TIME_SEC until reach the goal
    fn execute_step(&mut self, grid: &mut Grid, delta_time: f64) {
        if !self.sim_coordinator.is_ready_to_execute(delta_time) {
            return;
        }

        if let Some(current) = self.title_processing_queue.pop_front() {
            self.sim_coordinator.increase_step_count();
            let goal = grid.goal_title.unwrap();

            if self.sim_coordinator.is_goal_reached(current, goal) {
                self.handle_goal_reached(grid);
                return;
            }

            grid.mark_visited(current);
            let neighboring_titles = grid.get_neighbors(current);

            for neighbor_title in neighboring_titles {
                if !self.visited_titles.contains(&neighbor_title) {
                    grid.mark_process(neighbor_title);
                    self.visited_titles.push(neighbor_title);
                    self.title_path_mapping
                        .insert(Some(neighbor_title), Some(current));

                    // Early exit
                    if self.sim_coordinator.is_goal_reached(current, goal) {
                        self.handle_goal_reached(grid);
                        return;
                    }
                    self.title_processing_queue.push_back(neighbor_title);
                }
            }
        } else {
            self.sim_coordinator.is_processing = false;
        }
    }

    /// # reset
    /// Reset the algorithm processing
    fn reset(&mut self, grid: &mut Grid) {
        *self = Bfs::default();
        grid.reset();
    }

    /// # has_completed
    /// Check if processing is done
    fn has_completed(&self) -> bool {
        return self.sim_coordinator.has_completed;
    }
}

#[cfg(test)]
mod unit_test {

    use super::*;
    #[test]
    fn bfs_start() {
        let mut bfs = Bfs::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);

        assert!(bfs.start(&mut grid).is_err());

        let start = TitleCoords { x: 0, y: 0 };
        let end = TitleCoords { x: 1, y: 1 };
        grid.start_title = Some(start);
        grid.goal_title = Some(end);

        assert!(bfs.start(&mut grid).is_ok());

        assert_eq!(bfs.title_processing_queue.len(), 1);
        assert_eq!(bfs.title_processing_queue[0], start);
        assert_eq!(bfs.visited_titles.len(), 1);
        assert_eq!(bfs.visited_titles[0], start);
        assert!(*bfs.title_path_mapping.get(&Some(start)).unwrap() == None);
        assert!(!bfs.has_completed());
    }

    #[test]
    fn bfs_update_one_step() {
        let mut bfs = Bfs::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);
        assert!(bfs.start(&mut grid).is_err());

        let start = TitleCoords { x: 3, y: 3 };
        grid.start_title = Some(start);
        grid.goal_title = Some(TitleCoords { x: 10, y: 10 });
        assert!(bfs.start(&mut grid).is_ok());

        // After update
        //    [ ]
        // [ ][s][ ]
        //    [ ]

        bfs.execute_step(&mut grid, ONE_ITERATION_TIME_SEC);

        let exp_neighbors = [
            TitleCoords { x: 3, y: 2 },
            TitleCoords { x: 3, y: 4 },
            TitleCoords { x: 2, y: 3 },
            TitleCoords { x: 4, y: 3 },
        ];

        let expected_visited_tiles = [
            start,
            exp_neighbors[0],
            exp_neighbors[1],
            exp_neighbors[2],
            exp_neighbors[3],
        ];

        assert_eq!(bfs.visited_titles.len(), 5);
        for id in 0..bfs.visited_titles.len() {
            assert_eq!(expected_visited_tiles[id], bfs.visited_titles[id]);
        }

        assert_eq!(bfs.title_processing_queue.len(), 4);
        for id in 0..bfs.title_processing_queue.len() {
            assert_eq!(exp_neighbors[id], bfs.title_processing_queue[id]);
        }

        assert_eq!(bfs.title_path_mapping.len(), 5);
        assert_eq!(bfs.sim_coordinator.steps, 1);
        assert!(bfs.sim_coordinator.is_processing);
        assert!(!bfs.has_completed());
    }

    #[test]
    fn bfs_reset() {
        let mut bfs = Bfs::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);
        let start = TitleCoords { x: 0, y: 0 };
        let end = TitleCoords { x: 1, y: 1 };
        grid.start_title = Some(start);
        grid.goal_title = Some(end);

        assert!(bfs.start(&mut grid).is_ok());
        bfs.execute_step(&mut grid, ONE_ITERATION_TIME_SEC);
        bfs.reset(&mut grid);

        assert!(bfs.title_processing_queue.is_empty());
        assert!(bfs.visited_titles.is_empty());
        assert!(bfs.path.is_empty());
        assert!(bfs.title_path_mapping.is_empty());
        assert!(!bfs.sim_coordinator.is_processing);
        assert_eq!(bfs.sim_coordinator.steps, 0);
        assert_eq!(bfs.sim_coordinator.accumulated_time, 0.0);
    }
}
