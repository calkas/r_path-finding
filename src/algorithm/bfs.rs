use crate::map::grid::{Grid, TitleCoords};
use std::collections::{HashMap, VecDeque};

const ONE_ITERATION_TIME_SEC: f64 = 0.3;
const POSSIBLE_DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Up, Down, Left, Right

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
        println!(" * Path len {}", self.path.len());
        println!(" * Done in {} steps", self.steps);
        println!("FINISH");
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
            current_title = *self
                .title_path_mapping
                .get(&current_title)
                .expect("Title coord does not exist");
        }

        for element in self.path.iter() {
            grid.set_trace_back_path(*element);
        }
    }

    pub fn start(&mut self, grid: &mut Grid) {
        if grid.start_title.is_none() || grid.goal_title.is_none() {
            println!("User did not set the start or end point");
            return;
        }
        println!("..::BFS Algorithm:...");
        println!("Starts...");
        self.queue_of_titles.push_back(grid.start_title.unwrap());
        self.visited_titles.push(grid.start_title.unwrap());
        self.title_path_mapping
            .insert(Some(grid.start_title.unwrap()), None);
        self.is_processing = true;
    }

    pub fn update(&mut self, grid: &mut Grid, delta_time: f64) {
        if !self.is_processing || !self.should_iterate(delta_time) {
            return;
        }

        if let Some(current_title) = self.queue_of_titles.pop_front() {
            self.steps += 1;

            if self.is_goal_reached(current_title, grid.goal_title.unwrap()) {
                self.build_solution_path(grid);
                self.display_statistics();
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

                    if self.is_goal_reached(next_title, grid.goal_title.unwrap()) {
                        self.build_solution_path(grid);
                        self.display_statistics();
                        return;
                    }
                    self.queue_of_titles.push_back(next_title);
                }
            }
        } else {
            self.is_processing = false;
        }
    }
}
