use super::{Algorithm, AlgorithmError, Measurable, Pathfinder, SimulationCoordinator};
use crate::{
    algorithm::get_statistics,
    map::{grid::Grid, TitleCoords},
};
use std::collections::VecDeque;

/// # Breadth-First Search Algorithm
/// The algorithm explores titles equally in all directions.
#[derive(Default)]
pub struct Bfs {
    title_processing_queue: VecDeque<TitleCoords>,
    visited_titles: Vec<TitleCoords>,
    sim_coordinator: SimulationCoordinator,
    path_finder: Pathfinder,
}

impl Measurable for Bfs {
    fn output_statistics(&self) -> String {
        get_statistics(
            self.name().as_str(),
            self.path_finder.get_path().len(),
            self.sim_coordinator.steps,
            self.visited_titles.len(),
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
        if let Some(current) = self.title_processing_queue.pop_front() {
            self.sim_coordinator.increase_step_count();
            let goal = grid.goal_title.unwrap();
            let start = grid.start_title.unwrap();

            if self.sim_coordinator.process_goal_reached(current, goal) {
                self.path_finder.reconstruct_path(start, goal);
                for element in self.path_finder.get_path().iter() {
                    grid.set_trace_back_path(*element);
                }
                return;
            }
            grid.mark_visited(current);
            let neighboring_titles = grid.get_neighbors(current);

            for neighbor_title in neighboring_titles {
                if !self.visited_titles.contains(&neighbor_title) {
                    grid.mark_process(neighbor_title);
                    self.visited_titles.push(neighbor_title);
                    self.path_finder.add_to_path(neighbor_title, Some(current));
                    self.title_processing_queue.push_back(neighbor_title);
                }
            }
            // Check if goal is unreachable
            if self.title_processing_queue.is_empty() {
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
        *self = Bfs::default();
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
        "Breadth First Search".to_string()
    }
}

#[cfg(test)]
mod unit_test {

    use super::*;
    use crate::algorithm::ONE_ITERATION_TIME_SEC;
    #[test]
    fn bfs_start() {
        let mut bfs = Bfs::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);

        assert!(bfs.start(&mut grid).is_err());

        let start = TitleCoords { x: 0, y: 0 };

        grid.start_title = Some(start);
        grid.goal_title = Some(TitleCoords { x: 1, y: 1 });

        assert!(bfs.start(&mut grid).is_ok());

        assert_eq!(bfs.title_processing_queue[0], start);
        assert_eq!(bfs.visited_titles[0], start);
        assert_eq!(*bfs.path_finder.path.get(&start).unwrap(), None);
    }

    #[test]
    fn bfs_update_one_step() {
        let mut bfs = Bfs::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);

        let start = TitleCoords { x: 3, y: 3 };
        grid.start_title = Some(start);
        grid.goal_title = Some(TitleCoords { x: 10, y: 10 });

        let _ = bfs.start(&mut grid);

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
        for (id, item) in expected_visited_tiles.iter().enumerate() {
            assert_eq!(*item, bfs.visited_titles[id]);
        }

        assert_eq!(bfs.title_processing_queue.len(), 4);

        for (id, item) in exp_neighbors.iter().enumerate() {
            assert_eq!(*item, bfs.title_processing_queue[id]);
        }

        assert!(!bfs.has_completed());
    }

    #[test]
    fn bfs_reset() {
        let mut bfs = Bfs::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);
        grid.start_title = Some(TitleCoords { x: 0, y: 0 });
        grid.goal_title = Some(TitleCoords { x: 1, y: 1 });

        let _ = bfs.start(&mut grid);
        bfs.execute_step(&mut grid, ONE_ITERATION_TIME_SEC);
        bfs.reset(&mut grid);

        assert!(bfs.title_processing_queue.is_empty());
        assert!(bfs.visited_titles.is_empty());
    }
}
