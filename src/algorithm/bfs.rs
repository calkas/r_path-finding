use crate::map::grid::Grid;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn bfs_find_goal(grid: &mut Grid) {
    if grid.start.is_none() || grid.end.is_none() {
        println!("User did not set the start or end point");
        return;
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(grid.start.unwrap());

    let mut visited: Vec<(usize, usize)> = Vec::new();

    // bo x--->
    //    |
    //
    let possible_directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; //Up, Down, Left, Right

    while !queue.is_empty() {
        let current_title = queue.pop_front().unwrap();

        if current_title == grid.end.unwrap() {
            break;
        }

        //Go to 4 directions
        for direction in possible_directions.iter() {
            let posible_title = (
                current_title.0 as i32 + direction.0,
                current_title.1 as i32 + direction.1,
            );

            if posible_title.0 < 0
                || posible_title.0 >= grid.columns as i32
                || posible_title.1 < 0
                || posible_title.1 >= grid.rows as i32
            {
                continue;
            }

            let next_title = (posible_title.0 as usize, posible_title.1 as usize);
            if !visited.contains(&next_title) && !grid.is_obstacle(next_title.0, next_title.1) {
                grid.mark_visited(next_title.0, next_title.1);
                visited.push(next_title);
                queue.push_back(next_title);
            }
        }
    }
}
