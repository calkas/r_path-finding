pub mod algorithm;
mod map;

use algorithm::{Algorithm, AlgorithmError};
use map::grid::Grid;
use map::Title;
use piston_window::*;
use std::path::Path;

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
mod render_utils {
    use super::*;
    pub fn draw_text(glyph: &mut Glyphs, c: Context, g: &mut G2d, text: &str) {
        let transform = c.transform.trans(410.0, 50.0);
        for (line_number, line) in text.lines().enumerate() {
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 16)
                .draw(
                    line,
                    glyph,
                    &c.draw_state,
                    transform.trans(0.0, line_number as f64 * 15.0),
                    g,
                )
                .unwrap();
        }
    }
}
pub struct App {
    window: PistonWindow,
    grid: Grid,
    path_finding_algorithm: Box<dyn Algorithm>,
    mouse_action_fsm: fsm::MouseActionState,
}

impl App {
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
        let window: PistonWindow = WindowSettings::new("R-PathFinder", [700.0, 480.0])
            .build()
            .unwrap();

        let grid = Grid::new(0, 0, 400, 400, 20);

        Self {
            window,
            grid,
            path_finding_algorithm: algorithm,
            mouse_action_fsm: fsm::MouseActionState::new(),
        }
    }

    /// # run
    /// Run application/simulation
    pub fn run(&mut self) {
        let mut mouse_screen_position = [0.0, 0.0];
        let mut glyph = self.load_font_asset();

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

            e.update(|args: &UpdateArgs| {
                self.update_simulation_state(args);
            });

            self.window.draw_2d(&e, |c, g, device| {
                clear([0.5, 0.5, 0.5, 1.0], g);

                if self.path_finding_algorithm.has_completed() {
                    render_utils::draw_text(
                        &mut glyph,
                        c,
                        g,
                        &self.path_finding_algorithm.output_statistics(),
                    );
                }

                self.grid.render(&c, g);
                glyph.factory.encoder.flush(device);
            });
        }
    }

    fn load_font_asset(&mut self) -> Glyphs {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let font_dest_path = Path::new(&out_dir).join("assets/fonts/Roboto-Bold.ttf");
        let glyph = self.window.load_font(font_dest_path).unwrap();
        glyph
    }

    fn update_simulation_state(&mut self, args: &UpdateArgs) {
        if self.path_finding_algorithm.has_completed() {
            return;
        }
        self.path_finding_algorithm
            .execute_step(&mut self.grid, args.dt);
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
                let s = self.path_finding_algorithm.start(&mut self.grid);

                self.handle_algorithm_error(s);
                self.mouse_action_fsm = self.mouse_action_fsm.next();
            }
            fsm::MouseActionState::EndSimulation => {}
        }
    }

    fn reset_simulation(&mut self) {
        println!("Reset simulation");
        self.mouse_action_fsm = self.mouse_action_fsm.reset();
        self.path_finding_algorithm.reset(&mut self.grid);
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
