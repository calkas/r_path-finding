pub mod algorithm;
mod map;
mod render_utils;

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

pub mod application_message {
    pub const WELCOME: &str = "..::R-PATH-FINDER::..\n\n - 1-click left mouse button sets start\n\n - 2-click left mouse button sets goal\n\n - right mouse button sets obstacle\n\n - 3-click left mouse button starts\n  the simulation\n\n - Esc - restart simulation";
    pub const SIMULATION_STARTS: &str = "Simulation Starts...";
    pub const DONE: &str = "Done";
}

pub struct App<'a> {
    window: PistonWindow,
    grid: Grid,
    path_finding_algorithm: Box<dyn Algorithm>,
    mouse_action_fsm: fsm::MouseActionState,
    output_log: &'a str,
}

impl App<'_> {
    /// # new
    /// Create a new instance of application.
    ///
    /// The application uses the strategy pattern to set a specific algorithm for pathfinding.
    ///
    /// ## Example:
    /// ```
    /// use r_path_finder::algorithm::{bfs::Bfs, Algorithm};
    /// use r_path_finder::App;
    ///
    /// let bfs: Box<dyn Algorithm> = Box::new(Bfs::default());
    ///
    /// ```
    pub fn new(algorithm: Box<dyn Algorithm>) -> Self {
        let title_window = "R-PathFinder - ".to_string() + &algorithm.name();

        let window: PistonWindow = WindowSettings::new(title_window, [700.0, 480.0])
            .build()
            .unwrap();

        let grid = Grid::new(0, 0, 400, 400, 20);

        Self {
            window,
            grid,
            path_finding_algorithm: algorithm,
            mouse_action_fsm: fsm::MouseActionState::new(),
            output_log: application_message::WELCOME,
        }
    }

    /// # run
    /// Run application/simulation
    pub fn run(&mut self) {
        let mut mouse_screen_position = [0.0, 0.0];
        let mut glyph = self.load_font_asset();
        let mut is_drawing_locked = false;

        while let Some(e) = self.window.next() {
            if let Some(pos) = e.mouse_cursor_args() {
                mouse_screen_position = pos;
                if is_drawing_locked {
                    self.grid
                        .on_mouse_clicked(&mouse_screen_position, Title::Obstacle);
                }
            }

            if let Some(Button::Mouse(button)) = e.press_args() {
                match button {
                    MouseButton::Left => self.handle_mouse_action(mouse_screen_position),
                    MouseButton::Right => {
                        is_drawing_locked = true;
                        self.grid
                            .on_mouse_clicked(&mouse_screen_position, Title::Obstacle);
                    }
                    _ => (),
                }
            }

            if let Some(Button::Mouse(button)) = e.release_args() {
                match button {
                    MouseButton::Right => {
                        is_drawing_locked = false;
                    }
                    _ => (),
                }
            }

            if let Some(Button::Keyboard(Key::Escape)) = e.press_args() {
                self.reset_simulation();
            }

            e.update(|args: &UpdateArgs| {
                self.update_simulation_state(args);
            });

            self.window.draw_2d(&e, |c, g, device| {
                clear([0.5, 0.5, 0.5, 1.0], g);

                render_utils::draw_text(
                    self.output_log,
                    [410.0, 50.0],
                    16,
                    render_utils::color::BLACK,
                    &mut glyph,
                    &c,
                    g,
                );

                if self.path_finding_algorithm.has_completed() {
                    render_utils::draw_text(
                        &self.path_finding_algorithm.output_statistics(),
                        [410.0, 100.0],
                        16,
                        render_utils::color::BLACK,
                        &mut glyph,
                        &c,
                        g,
                    );
                }

                self.grid.render(&c, g);
                glyph.factory.encoder.flush(device);
            });
        }
    }

    fn load_font_asset(&mut self) -> Glyphs {
        let font_source_path = std::env::current_dir()
            .unwrap()
            .join("assets/fonts/Roboto-Bold.ttf");
        self.window.load_font(font_source_path).unwrap()
    }

    fn update_simulation_state(&mut self, args: &UpdateArgs) {
        if self.path_finding_algorithm.has_completed() {
            self.output_log = application_message::DONE;
            return;
        }
        self.path_finding_algorithm
            .execute_step(&mut self.grid, args.dt);
    }

    fn handle_mouse_action(&mut self, mouse_pos: [f64; 2]) {
        match self.mouse_action_fsm {
            fsm::MouseActionState::SetStartPoint => {
                self.grid.on_mouse_clicked(&mouse_pos, Title::Start);
                if self.grid.start_title.is_some() {
                    self.mouse_action_fsm = self.mouse_action_fsm.next();
                }
            }

            fsm::MouseActionState::SetEndPoint => {
                self.grid.on_mouse_clicked(&mouse_pos, Title::End);
                if self.grid.goal_title.is_some() {
                    self.mouse_action_fsm = self.mouse_action_fsm.next();
                }
            }
            fsm::MouseActionState::StartSimulation => {
                let s = self.path_finding_algorithm.start(&mut self.grid);
                self.handle_algorithm_error(s);
                self.mouse_action_fsm = self.mouse_action_fsm.next();
            }
            fsm::MouseActionState::EndSimulation => {}
        }
    }

    fn reset_simulation(&mut self) {
        self.output_log = application_message::WELCOME;
        self.mouse_action_fsm = self.mouse_action_fsm.reset();
        self.path_finding_algorithm.reset(&mut self.grid);
    }

    fn handle_algorithm_error(&mut self, status: Result<(), AlgorithmError>) {
        match status {
            Err(AlgorithmError::InvalidInputData) => {
                println!("User did not set the start or end point")
            }
            Ok(_) => {
                self.output_log = application_message::SIMULATION_STARTS;
            }
        }
    }
}
