use super::Algorithm;
use crate::map::grid::Grid;
use crate::map::TitleCoords;
use std::collections::{HashMap, VecDeque};

const ONE_ITERATION_TIME_SEC: f64 = 0.3;

/// FYI the coordinate system is
///
/// (0,0)----> x    <br>
/// |               <br>
/// |               <br>
/// V y             <br>
const POSSIBLE_DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Up, Down, Left, Right

/// # Breadth-First Search Algorithm
pub struct Bfs {
    queue_of_titles: VecDeque<TitleCoords>,
    visited_titles: Vec<TitleCoords>,
    path: Vec<TitleCoords>,
    title_path_mapping: HashMap<Option<TitleCoords>, Option<TitleCoords>>,
    is_processing: bool,
    steps: u32,
    accumulated_time: f64,
}

impl Default for Bfs {
    fn default() -> Self {
        Self {
            queue_of_titles: VecDeque::default(),
            visited_titles: Vec::default(),
            path: Vec::default(),
            title_path_mapping: HashMap::default(),
            is_processing: false,
            steps: 0,
            accumulated_time: 0.0,
        }
    }
}

impl Bfs {
    fn is_goal_reached(&mut self, current_title: TitleCoords, goal_title: TitleCoords) -> bool {
        if current_title != goal_title {
            return false;
        }
        self.is_processing = false;
        true
    }

    fn display_statistics(&self) {
        println!("- Finish");
        println!("- Statistics:");
        println!(" * Path length: {}", self.path.len());
        println!(" * Steps taken: {}", self.steps);
        println!(" * Visited nodes: {}", self.visited_titles.len());
        println!(
            " * Time per iteration: {:.2} seconds",
            ONE_ITERATION_TIME_SEC
        );
        println!(
            " * Total time: {:.2} seconds",
            self.steps as f64 * ONE_ITERATION_TIME_SEC
        );
        println!("- Done");
    }

    fn should_iterate(&mut self, delta_time: f64) -> bool {
        self.accumulated_time += delta_time;
        if self.accumulated_time < ONE_ITERATION_TIME_SEC {
            return false;
        }
        self.accumulated_time = 0.0;
        true
    }

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
        self.build_solution_path(grid);
        self.display_statistics();
    }
}

impl Algorithm for Bfs {
    /// # start
    /// BFS Algorithm starts.
    ///
    /// Init the algorithm values
    fn start(&mut self, grid: &mut Grid) -> bool {
        if grid.start_title.is_none() || grid.goal_title.is_none() {
            println!("User did not set the start or end point");
            return false;
        }
        println!("..::BFS Algorithm:...");
        println!("- Starts");
        self.queue_of_titles.push_back(grid.start_title.unwrap());
        self.visited_titles.push(grid.start_title.unwrap());
        self.title_path_mapping
            .insert(Some(grid.start_title.unwrap()), None);
        self.is_processing = true;
        true
    }

    /// # update
    /// Algorithm processing update every ONE_ITERATION_TIME_SEC until reach the goal
    fn update(&mut self, grid: &mut Grid, delta_time: f64) {
        if !self.is_processing || !self.should_iterate(delta_time) {
            return;
        }

        if let Some(current_title) = self.queue_of_titles.pop_front() {
            self.steps += 1;

            if self.is_goal_reached(current_title, grid.goal_title.unwrap()) {
                self.handle_goal_reached(grid);
                return;
            }

            for direction in POSSIBLE_DIRECTIONS.iter() {
                let coord_x = current_title.x.checked_add_signed(direction.0);
                let coord_y = current_title.y.checked_add_signed(direction.1);

                if coord_x.is_none() || coord_y.is_none() {
                    continue;
                }

                let next_title = TitleCoords {
                    x: coord_x.unwrap(),
                    y: coord_y.unwrap(),
                };

                if !self.visited_titles.contains(&next_title)
                    && grid.is_within_bounds(next_title)
                    && !grid.is_obstacle(next_title)
                {
                    grid.mark_visited(next_title);
                    self.visited_titles.push(next_title);
                    self.title_path_mapping
                        .insert(Some(next_title), Some(current_title));

                    if self.is_goal_reached(current_title, grid.goal_title.unwrap()) {
                        self.handle_goal_reached(grid);
                        return;
                    }
                    self.queue_of_titles.push_back(next_title);
                }
            }
        } else {
            self.is_processing = false;
        }
    }

    /// # reset
    /// Reset the algorithm processing
    fn reset(&mut self, grid: &mut Grid) {
        *self = Bfs::default();
        grid.reset();
    }
}

#[cfg(test)]

mod unit_test {

    use super::*;
    #[test]
    fn bfs_start() {
        let mut bfs = Bfs::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);

        assert!(bfs.start(&mut grid) == false);

        let start = TitleCoords { x: 0, y: 0 };
        let end = TitleCoords { x: 1, y: 1 };
        grid.start_title = Some(start);
        grid.goal_title = Some(end);

        assert!(bfs.start(&mut grid));

        assert_eq!(bfs.queue_of_titles.len(), 1);
        assert_eq!(bfs.queue_of_titles[0], start);
        assert_eq!(bfs.visited_titles.len(), 1);
        assert_eq!(bfs.visited_titles[0], start);
        assert!(*bfs.title_path_mapping.get(&Some(start)).unwrap() == None);
        assert!(bfs.is_processing);
    }

    #[test]
    fn bfs_update_one_step() {
        let mut bfs = Bfs::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);

        assert!(bfs.start(&mut grid) == false);

        let start = TitleCoords { x: 3, y: 3 };
        grid.start_title = Some(start);
        grid.goal_title = Some(TitleCoords { x: 10, y: 10 });

        assert!(bfs.start(&mut grid));

        // After update
        //    [ ]
        // [ ][s][ ]
        //    [ ]

        bfs.update(&mut grid, ONE_ITERATION_TIME_SEC);

        let mut expected_directions: Vec<TitleCoords> = Vec::new();
        let mut expected_visited_tiles: Vec<TitleCoords> = Vec::new();
        expected_visited_tiles.push(start);

        for dir in POSSIBLE_DIRECTIONS.iter() {
            let coord_x = start.x.checked_add_signed(dir.0);
            let coord_y = start.y.checked_add_signed(dir.1);

            if coord_x.is_none() || coord_y.is_none() {
                continue;
            }

            expected_directions.push(TitleCoords {
                x: coord_x.unwrap(),
                y: coord_y.unwrap(),
            });

            expected_visited_tiles.push(TitleCoords {
                x: coord_x.unwrap(),
                y: coord_y.unwrap(),
            });
        }

        assert_eq!(bfs.queue_of_titles.len(), 4);
        for id in 0..bfs.queue_of_titles.len() {
            assert_eq!(expected_directions[id], bfs.queue_of_titles[id]);
        }

        assert_eq!(bfs.visited_titles.len(), 5);
        for id in 0..bfs.visited_titles.len() {
            assert_eq!(expected_visited_tiles[id], bfs.visited_titles[id]);
        }

        assert_eq!(bfs.title_path_mapping.len(), 5);
        assert_eq!(bfs.steps, 1);
        assert_eq!(bfs.accumulated_time, 0.0);
        assert!(bfs.is_processing);
    }

    #[test]
    fn bfs_reset() {
        let mut bfs = Bfs::default();
        let mut grid = Grid::new(0, 0, 10, 10, 1);
        let start = TitleCoords { x: 0, y: 0 };
        let end = TitleCoords { x: 1, y: 1 };
        grid.start_title = Some(start);
        grid.goal_title = Some(end);

        bfs.start(&mut grid);
        bfs.update(&mut grid, ONE_ITERATION_TIME_SEC);
        bfs.reset(&mut grid);

        assert!(bfs.queue_of_titles.is_empty());
        assert!(bfs.visited_titles.is_empty());
        assert!(bfs.path.is_empty());
        assert!(bfs.title_path_mapping.is_empty());
        assert!(!bfs.is_processing);
        assert_eq!(bfs.steps, 0);
        assert_eq!(bfs.accumulated_time, 0.0);
    }
}
