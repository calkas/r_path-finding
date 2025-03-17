pub mod algorithm;
mod map;
use std::{cell::RefCell, rc::Rc};

use algorithm::{Algorithm, AlgorithmError};
use map::grid::Grid;
use map::Title;
use piston_window::*;

mod fsm {
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum MouseActionState {
        SetStartPoint,
        SetEndPoint,
        StartSimulation,
        EndSimulation,
    }

    impl MouseActionState {
        pub fn new() -> Self {
            Self::SetStartPoint
        }
        pub fn next(self) -> Self {
            match self {
                Self::SetStartPoint => Self::SetEndPoint,
                Self::SetEndPoint => Self::StartSimulation,
                Self::StartSimulation => Self::EndSimulation,
                Self::EndSimulation => Self::EndSimulation,
            }
        }
        pub fn reset(self) -> Self {
            Self::SetStartPoint
        }
    }
}

pub struct App {
    window: PistonWindow,
    grid: Grid,
    path_finding_algorithm: Rc<RefCell<dyn Algorithm>>,
    mouse_action_fsm: fsm::MouseActionState,
    should_update_simulation: bool,
}

impl App {
    /// # new
    /// Create a new instance of application.
    ///
    /// Application uses specific algorithm for path finding
    /// ## Example:
    /// ```
    ///   use r_path_finder::algorithm;
    ///   use std::cell::RefCell;
    ///   use std::rc::Rc;
    ///
    ///   let bfs = algorithm::bfs::Bfs::default();
    ///   let algorithm: Rc<RefCell<dyn algorithm::Algorithm>> = Rc::new(RefCell::new(bfs));
    /// ```
    pub fn new(algorithm: Rc<RefCell<dyn Algorithm>>) -> Self {
        let window: PistonWindow = WindowSettings::new("R-PathFinder", [640.0, 480.0])
            .build()
            .unwrap();

        let grid = Grid::new(0, 0, 400, 400, 20);

        Self {
            window,
            grid,
            path_finding_algorithm: algorithm,
            mouse_action_fsm: fsm::MouseActionState::new(),
            should_update_simulation: true,
        }
    }

    /// # run
    /// Run application/simulation
    pub fn run(&mut self) {
        let mut mouse_screen_position = [0.0, 0.0];

        while let Some(e) = self.window.next() {
            if let Some(pos) = e.mouse_cursor_args() {
                mouse_screen_position = pos;
            }

            if let Some(Button::Mouse(button)) = e.press_args() {
                match button {
                    MouseButton::Left => self.handle_mouse_action(mouse_screen_position),
                    MouseButton::Right => {
                        self.grid
                            .on_mouse_clicked(&mouse_screen_position, Title::Obstacle);
                    }
                    _ => (),
                }
            }

            if let Some(Button::Keyboard(Key::Escape)) = e.press_args() {
                self.reset_simulation();
            }

            e.update(|args| {
                if self.should_update_simulation {
                    self.update_simulation_state(args);
                }
            });

            self.window.draw_2d(&e, |c, g, _| {
                clear([0.5, 0.5, 0.5, 1.0], g);
                self.grid.render(&c, g);
            });
        }
    }

    fn update_simulation_state(&mut self, args: &UpdateArgs) {
        if self.path_finding_algorithm.borrow().has_completed() {
            println!(
                "{}",
                self.path_finding_algorithm.borrow().output_statistics()
            );
            self.should_update_simulation = false;
        } else {
            self.path_finding_algorithm
                .borrow_mut()
                .execute_step(&mut self.grid, args.dt);
        }
    }

    fn handle_mouse_action(&mut self, mouse_pos: [f64; 2]) {
        match self.mouse_action_fsm {
            fsm::MouseActionState::SetStartPoint => {
                self.grid.on_mouse_clicked(&mouse_pos, Title::Start);
                self.mouse_action_fsm = self.mouse_action_fsm.next();
            }
            fsm::MouseActionState::SetEndPoint => {
                self.grid.on_mouse_clicked(&mouse_pos, Title::End);
                self.mouse_action_fsm = self.mouse_action_fsm.next();
            }
            fsm::MouseActionState::StartSimulation => {
                let s = self
                    .path_finding_algorithm
                    .borrow_mut()
                    .start(&mut self.grid);

                self.handle_algorithm_error(s);
                self.mouse_action_fsm = self.mouse_action_fsm.next();
            }
            fsm::MouseActionState::EndSimulation => {}
        }
    }

    fn reset_simulation(&mut self) {
        println!("Reset simulation");
        self.should_update_simulation = true;
        self.mouse_action_fsm = self.mouse_action_fsm.reset();
        self.path_finding_algorithm
            .borrow_mut()
            .reset(&mut self.grid);
    }

    fn handle_algorithm_error(&self, status: Result<(), AlgorithmError>) {
        match status {
            Err(AlgorithmError::InvalidInputData) => {
                println!("User did not set the start or end point")
            }
            Ok(_) => {
                println!("Simulation Starts...")
            }
        }
    }
}
