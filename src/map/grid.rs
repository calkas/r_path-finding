use super::{Title, TitleCoords};
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

/// FYI the coordinate system is
///
/// (0,0)----> x    <br>
/// |               <br>
/// |               <br>
/// V y             <br>
pub const POSSIBLE_DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Up, Down, Left, Right

/// # Grid
/// Grid of titles used for path-finding algorithms
pub struct Grid {
    pub rows: u32,
    pub columns: u32,
    title_size: u32,
    offset: (u32, u32),
    titles: Vec<Vec<Title>>,
    pub start_title: Option<TitleCoords>,
    pub goal_title: Option<TitleCoords>,
}

impl Grid {
    /// # new
    /// Create a new grid with dim(width x height) with offset(x,y) and title size
    pub fn new(x: u32, y: u32, width: u32, height: u32, title_size: u32) -> Self {
        let mut titles: Vec<Vec<Title>> = Vec::new();
        let columns = width / title_size;
        let rows = height / title_size;

        for _ in 0..rows {
            let mut row_titles = Vec::new();
            for _ in 0..columns {
                row_titles.push(Title::Normal { was_visited: false });
            }
            titles.push(row_titles);
        }

        let offset = (x, y);

        Self {
            rows,
            columns,
            title_size,
            offset,
            titles,
            start_title: None,
            goal_title: None,
        }
    }

    /// # on_mouse_clicked
    /// Mouse click event to setup start,end and additional obstacle title
    pub fn on_mouse_clicked(&mut self, mouse_positon: &[f64; 2], title: Title) {
        if mouse_positon[0] >= self.offset.0 as f64
            && mouse_positon[0] < (self.offset.0 + self.columns * self.title_size) as f64
            && mouse_positon[1] >= self.offset.1 as f64
            && mouse_positon[1] < (self.offset.1 + self.rows * self.title_size) as f64
        {
            let x = ((mouse_positon[0] - self.offset.0 as f64) / self.title_size as f64) as usize;
            let y = ((mouse_positon[1] - self.offset.1 as f64) / self.title_size as f64) as usize;

            if self.is_not_set(TitleCoords { x, y }) {
                match title {
                    Title::Start => self.start_title = Some(TitleCoords { x, y }),
                    Title::End => self.goal_title = Some(TitleCoords { x, y }),
                    _ => {}
                }
                self.titles[x][y] = title;
            }
        }
    }

    /// # get_neighbors
    /// Get title neighbors for current title
    pub fn get_neighbors(&mut self, current_coord: TitleCoords) -> Vec<TitleCoords> {
        let neighbors = POSSIBLE_DIRECTIONS
            .into_iter()
            .filter_map(|step_direction| {
                let coord_x = current_coord.x.checked_add_signed(step_direction.0)?;
                let coord_y = current_coord.y.checked_add_signed(step_direction.1)?;

                let neighbor_coordinates = TitleCoords {
                    x: coord_x,
                    y: coord_y,
                };

                if !self.is_within_bounds(neighbor_coordinates)
                    || self.is_obstacle(neighbor_coordinates)
                {
                    return None;
                }
                Some(neighbor_coordinates)
            })
            .collect();
        neighbors
    }
    /// # cost
    /// The movement cost - Is 1 for normal grid
    pub fn cost(&self, _current: TitleCoords, _next: TitleCoords) -> i32 {
        1
    }

    /// # heuristic
    /// Manhattan distance on a square grid
    pub fn heuristic(&self, current: TitleCoords, next: TitleCoords) -> i32 {
        let x_distance = (next.x as i32) - (current.x as i32);
        let y_distance = (next.y as i32) - (current.y as i32);
        x_distance.abs() + y_distance.abs()
    }

    /// # is_within_bounds
    /// Check if coordinate of title is in the grid
    pub fn is_within_bounds(&self, title_coords: TitleCoords) -> bool {
        (title_coords.x as u32) < self.columns && (title_coords.y as u32) < self.rows
    }

    /// # mark_visited
    /// Mark title visited
    pub fn mark_visited(&mut self, title_coords: TitleCoords) {
        if !self.is_within_bounds(title_coords) || self.is_title_start_or_end(title_coords) {
            return;
        }
        self.titles[title_coords.x][title_coords.y] = Title::Normal { was_visited: true };
    }

    /// # mark_process
    /// Mark title in processing state
    pub fn mark_process(&mut self, title_coords: TitleCoords) {
        if !self.is_within_bounds(title_coords) || self.is_title_start_or_end(title_coords) {
            return;
        }
        self.titles[title_coords.x][title_coords.y] = Title::Process;
    }

    /// # set_trace_back_path
    /// Set title to be a title path
    pub fn set_trace_back_path(&mut self, title_coords: TitleCoords) {
        if self.is_within_bounds(title_coords)
            && self.titles[title_coords.x][title_coords.y] == (Title::Normal { was_visited: true })
        {
            self.titles[title_coords.x][title_coords.y] = Title::Path;
        }
    }

    /// # is_obstacle
    /// Is current title is obstacle
    pub fn is_obstacle(&self, title_coords: TitleCoords) -> bool {
        if !self.is_within_bounds(title_coords) {
            return true;
        }
        self.titles[title_coords.x][title_coords.y] == Title::Obstacle
    }
    /// # reset
    /// Clear grid
    pub fn reset(&mut self) {
        for row in self.titles.iter_mut() {
            for title in row.iter_mut() {
                *title = Title::Normal { was_visited: false };
            }
        }
        self.start_title = None;
        self.goal_title = None;
    }

    /// # render
    /// Render current grid with titles
    pub fn render(&mut self, ctx: &Context, g: &mut G2d) {
        for x in 0..self.rows {
            for y in 0..self.columns {
                let title = &self.titles[x as usize][y as usize];
                let color = self.get_color_for_title(title);
                rectangle(
                    color,
                    [
                        ((x * self.title_size) + self.offset.0) as f64,
                        ((y * self.title_size) + self.offset.1) as f64,
                        (self.title_size - 2) as f64,
                        (self.title_size - 2) as f64,
                    ],
                    ctx.transform,
                    g,
                );
            }
        }
    }

    fn get_color_for_title(&self, title: &Title) -> Color {
        match *title {
            Title::Normal { was_visited: false } => [1.0, 0.0, 0.0, 1.0],
            Title::Normal { was_visited: true } => [0.0, 1.0, 0.0, 1.0],
            Title::Start => [1.0, 0.878, 0.0, 1.0],
            Title::End => [0.255, 0.706, 0.949, 1.0],
            Title::Obstacle => [0.569, 0.471, 0.365, 1.0],
            Title::Path => [0.0, 0.0, 1.0, 1.0],
            Title::Process => [0.45, 0.984, 0.84, 1.0],
        }
    }

    fn is_title_start_or_end(&self, title_coords: TitleCoords) -> bool {
        if self.titles[title_coords.x][title_coords.y] == Title::Start
            || self.titles[title_coords.x][title_coords.y] == Title::End
        {
            return true;
        }
        false
    }
    fn is_not_set(&self, title_coords: TitleCoords) -> bool {
        if self.titles[title_coords.x][title_coords.y] == (Title::Normal { was_visited: false }) {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn verify_grid_behavior() {
        let mut grid = Grid::new(0, 0, 50, 50, 10);
        let number_of_titles: usize = grid.titles.iter().map(|row| row.len()).sum();

        assert_eq!(25 as usize, number_of_titles);

        grid.on_mouse_clicked(&[15.0, 15.3], Title::Start);
        grid.on_mouse_clicked(&[21.4, 36.3], Title::End);
        grid.on_mouse_clicked(&[41.4, 20.3], Title::Obstacle);

        let start_title_coord = TitleCoords { x: 1, y: 1 };
        let end_title_coord = TitleCoords { x: 2, y: 3 };
        let obstacle_title_coord = TitleCoords { x: 4, y: 2 };
        let normal_title_coord = TitleCoords { x: 3, y: 3 };
        let out_of_bounds_title_coord = TitleCoords { x: 10, y: 10 };

        assert_eq!(
            Title::Start,
            grid.titles[start_title_coord.x][start_title_coord.y]
        );
        assert_eq!(grid.start_title, Some(start_title_coord));
        assert_eq!(
            Title::End,
            grid.titles[end_title_coord.x][end_title_coord.y]
        );
        assert_eq!(grid.goal_title, Some(end_title_coord));
        assert!(grid.is_obstacle(obstacle_title_coord));

        grid.mark_visited(normal_title_coord);
        assert!(grid.is_within_bounds(normal_title_coord));
        assert_eq!(
            Title::Normal { was_visited: true },
            grid.titles[normal_title_coord.x][normal_title_coord.y]
        );

        assert!(!grid.is_within_bounds(out_of_bounds_title_coord));

        grid.set_trace_back_path(normal_title_coord);
        assert_eq!(
            Title::Path,
            grid.titles[normal_title_coord.x][normal_title_coord.y]
        );
    }
    #[test]
    fn grid_reset() {
        let mut grid = Grid::new(0, 0, 5, 5, 1);

        grid.start_title = Some(TitleCoords { x: 1, y: 1 });
        grid.goal_title = Some(TitleCoords { x: 2, y: 3 });
        grid.mark_visited(TitleCoords { x: 3, y: 3 });
        grid.mark_visited(TitleCoords { x: 4, y: 1 });
        grid.mark_visited(TitleCoords { x: 4, y: 2 });

        grid.reset();

        assert!(grid.start_title.is_none());
        assert!(grid.goal_title.is_none());
        for row in grid.titles.iter() {
            for title in row.iter() {
                assert_eq!(*title, Title::Normal { was_visited: false });
            }
        }
    }
    #[test]
    fn calculate_neighbors() {
        let mut grid = Grid::new(0, 0, 10, 10, 1);

        grid.start_title = Some(TitleCoords { x: 5, y: 5 });
        grid.goal_title = Some(TitleCoords { x: 9, y: 9 });

        let start = grid.start_title.unwrap();

        let neighbors = grid.get_neighbors(start);

        let exp_neighbors = [
            TitleCoords { x: 5, y: 4 },
            TitleCoords { x: 5, y: 6 },
            TitleCoords { x: 4, y: 5 },
            TitleCoords { x: 6, y: 5 },
        ];

        assert_eq!(4, neighbors.len());
        for (id, actual) in neighbors.iter().enumerate() {
            assert_eq!(exp_neighbors[id], *actual);
        }
    }
}
