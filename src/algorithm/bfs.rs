use crate::map::grid::Grid;
use crate::map::grid::TitleCoords;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Bfs {
    queue_of_titles: VecDeque<TitleCoords>,
    visited_titles: Vec<TitleCoords>,
    is_processing: bool,
    steps: u32,
}

impl Default for Bfs {
    fn default() -> Self {
        Self {
            queue_of_titles: Default::default(),
            visited_titles: Default::default(),
            is_processing: false,
            steps: 0,
        }
    }
}

impl Bfs {
    fn is_goal_reached(&mut self, current_title: TitleCoords, goal_title: TitleCoords) -> bool {
        if current_title != goal_title {
            return false;
        }
        println!("Done in {} steps", self.steps);
        self.is_processing = false;
        return true;
    }

    pub fn start(&mut self, grid: &mut Grid) {
        if grid.start.is_none() || grid.end.is_none() {
            println!("User did not set the start or end point");
            return;
        }
        println!("BFS Algorithm starts..");
        self.queue_of_titles.push_back(grid.start.unwrap());
        self.is_processing = true;
    }

    pub fn update(&mut self, grid: &mut Grid, delta_time: f64) {
        if !self.is_processing {
            return;
        }

        static mut ACCUMULATED_TIME: f64 = 0.0;
        unsafe {
            ACCUMULATED_TIME += delta_time;
            if ACCUMULATED_TIME < 0.3 {
                return;
            }
            ACCUMULATED_TIME = 0.0;
        }

        if let Some(current_title) = self.queue_of_titles.pop_front() {
            let possible_directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Up, Down, Left, Right
            self.steps = self.steps + 1;

            for direction in possible_directions.iter() {
                let possible_title = (
                    current_title.x as i32 + direction.0,
                    current_title.y as i32 + direction.1,
                );

                if possible_title.0 < 0
                    || possible_title.0 >= grid.columns as i32
                    || possible_title.1 < 0
                    || possible_title.1 >= grid.rows as i32
                {
                    continue;
                }

                let next_title = TitleCoords {
                    x: possible_title.0 as usize,
                    y: possible_title.1 as usize,
                };

                if !self.visited_titles.contains(&next_title)
                    && !grid.is_obstacle(next_title.x, next_title.y)
                {
                    grid.mark_visited(next_title.x, next_title.y);
                    self.visited_titles.push(next_title);

                    if self.is_goal_reached(next_title, grid.end.unwrap()) {
                        return;
                    }
                    self.queue_of_titles.push_back(next_title);
                }
            }

            if self.is_goal_reached(current_title, grid.end.unwrap()) {
                return;
            }
        } else {
            self.is_processing = false;
        }
    }
}

// pub fn bfs_find_goal(grid: &mut Grid) {
//     if grid.start.is_none() || grid.end.is_none() {
//         println!("User did not set the start or end point");
//         return;
//     }

//     let mut queue: VecDeque<TitleCoords> = VecDeque::new();
//     queue.push_back(grid.start.unwrap());

//     let mut visited: Vec<TitleCoords> = Vec::new();

//     // bo x--->
//     //    |
//     //
//     let possible_directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; //Up, Down, Left, Right

//     while !queue.is_empty() {
//         let current_title = queue.pop_front().unwrap();

//         if current_title == grid.end.unwrap() {
//             break;
//         }

//         //Go to 4 directions
//         for direction in possible_directions.iter() {
//             let posible_title = (
//                 current_title.x as i32 + direction.0,
//                 current_title.y as i32 + direction.1,
//             );

//             if posible_title.0 < 0
//                 || posible_title.0 >= grid.columns as i32
//                 || posible_title.1 < 0
//                 || posible_title.1 >= grid.rows as i32
//             {
//                 continue;
//             }

//             let next_title = TitleCoords {
//                 x: posible_title.0 as usize,
//                 y: posible_title.1 as usize,
//             };
//             if !visited.contains(&next_title) && !grid.is_obstacle(next_title.x, next_title.y) {
//                 grid.mark_visited(next_title.x, next_title.y);
//                 visited.push(next_title);
//                 queue.push_back(next_title);
//             }
//         }
//     }
// }
