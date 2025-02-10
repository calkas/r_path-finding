use piston_window::rectangle;
use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

#[derive(PartialEq, Debug)]
pub enum Title {
    Normal { was_visited: bool },
    Start,
    End,
    Obstacle,
    Path,
}

pub struct Grid {
    pub rows: u32,
    pub columns: u32,
    title_size: u32,
    offset: (u32, u32),
    titles: Vec<Vec<Title>>,
    pub start: Option<(usize, usize)>,
    pub end: Option<(usize, usize)>,
}

impl Grid {
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
            start: None,
            end: None,
        }
    }

    pub fn on_mouse_clicked(&mut self, mouse_positon: &[f64; 2], title: Title) {
        if mouse_positon[0] >= self.offset.0 as f64
            && mouse_positon[0] < (self.offset.0 + self.columns * self.title_size) as f64
            && mouse_positon[1] >= self.offset.1 as f64
            && mouse_positon[1] < (self.offset.1 + self.rows * self.title_size) as f64
        {
            let x = ((mouse_positon[0] - self.offset.0 as f64) / self.title_size as f64) as usize;
            let y = ((mouse_positon[1] - self.offset.1 as f64) / self.title_size as f64) as usize;

            if title == Title::Start {
                self.start = Some((x, y));
            }

            if title == Title::End {
                self.end = Some((x, y));
            }

            self.titles[x][y] = title;
        }
    }

    pub fn mark_visited(&mut self, x: usize, y: usize) {
        if x as u32 >= self.columns || y as u32 >= self.rows {
            return;
        }

        if self.titles[x][y] == Title::Start || self.titles[x][y] == Title::End {
            return;
        }
        self.titles[x][y] = Title::Normal { was_visited: true };
    }

    pub fn set_trace_back_path(&mut self, x: usize, y: usize) {
        self.titles[x][y] = Title::Path;
    }
    pub fn is_obstacle(&self, x: usize, y: usize) -> bool {
        self.titles[x][y] == Title::Obstacle
    }

    fn get_color_for_title(&self, title: &Title) -> Color {
        match *title {
            Title::Normal { was_visited: false } => [1.0, 0.0, 0.0, 1.0],
            Title::Normal { was_visited: true } => [0.0, 1.0, 0.0, 1.0],
            Title::Start => [1.0, 0.878, 0.0, 1.0],
            Title::End => [0.255, 0.706, 0.949, 1.0],
            Title::Obstacle => [0.569, 0.471, 0.365, 1.0],
            Title::Path => [0.0, 0.0, 1.0, 1.0],
        }
    }

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
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn grid_creation() {
        let mut grid = Grid::new(0, 0, 50, 50, 10);
        let number_of_titles: usize = grid.titles.iter().map(|row| row.len()).sum();

        assert_eq!(25 as usize, number_of_titles);

        grid.mark_visited(3, 3);
        assert_eq!(Title::Normal { was_visited: true }, grid.titles[3][3]);

        grid.on_mouse_clicked(&[15.0, 15.3], Title::Start);
        grid.on_mouse_clicked(&[21.4, 36.3], Title::End);
        grid.on_mouse_clicked(&[41.4, 20.3], Title::Obstacle);

        assert_eq!(Title::Start, grid.titles[1][1]);
        assert_eq!(grid.start, Some((1, 1)));
        assert_eq!(Title::End, grid.titles[2][3]);
        assert_eq!(grid.end, Some((2, 3)));
        assert_eq!(Title::Obstacle, grid.titles[4][2]);
    }
}
