pub mod algorithm;
mod map;
use std::{cell::RefCell, rc::Rc};

use algorithm::Algorithm;
use map::grid::Grid;
use piston_window::*;

pub struct App {
    window: PistonWindow,
    grid: Grid,
    path_finding_algorithm: Rc<RefCell<dyn Algorithm>>,
}

impl App {
    /// # new
    /// Create a new instance of application.
    ///
    /// Application uses specific algorithm for path finding
    /// ## Example:
    /// ```
    ///   use r_path_finder::{algorithm, App};
    ///   use std::cell::RefCell;
    ///   use std::rc::Rc;
    ///
    ///   let bfs = algorithm::bfs::Bfs::default();
    ///   let algorithm: Rc<RefCell<dyn algorithm::Algorithm>> = Rc::new(RefCell::new(bfs));
    ///   let mut app = App::new(algorithm.clone());
    /// ```
    pub fn new(algorithm: Rc<RefCell<dyn Algorithm>>) -> Self {
        let window: PistonWindow = WindowSettings::new("Path Finding", [640.0, 480.0])
            .build()
            .unwrap();

        let grid = Grid::new(0, 0, 400, 400, 20);

        Self {
            window,
            grid,
            path_finding_algorithm: algorithm,
        }
    }

    /// # run
    /// Run application
    pub fn run(&mut self) {
        let mut left_click_count = 0;
        let mut mouse_pos = [0.0, 0.0];

        while let Some(e) = self.window.next() {
            if let Some(pos) = e.mouse_cursor_args() {
                mouse_pos = pos;
            }

            if let Some(Button::Mouse(button)) = e.press_args() {
                match button {
                    MouseButton::Left => {
                        if left_click_count == 0 {
                            self.grid
                                .on_mouse_clicked(&mouse_pos, map::grid::Title::Start);
                        } else if left_click_count == 1 {
                            self.grid
                                .on_mouse_clicked(&mouse_pos, map::grid::Title::End);
                        } else if left_click_count == 2 {
                            self.path_finding_algorithm
                                .borrow_mut()
                                .start(&mut self.grid);
                        }
                        left_click_count += 1;
                    }
                    MouseButton::Right => {
                        self.grid
                            .on_mouse_clicked(&mouse_pos, map::grid::Title::Obstacle);
                    }
                    _ => (),
                }
            }

            if let Some(Button::Keyboard(Key::Escape)) = e.press_args() {
                println!("Reset");
                left_click_count = 0;
                self.path_finding_algorithm
                    .borrow_mut()
                    .reset(&mut self.grid);
            }

            e.update(|args| {
                self.path_finding_algorithm
                    .borrow_mut()
                    .update(&mut self.grid, args.dt);
            });

            self.window.draw_2d(&e, |c, g, _| {
                clear([0.5, 0.5, 0.5, 1.0], g);
                self.grid.render(&c, g);
            });
        }
    }
}
