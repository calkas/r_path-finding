pub mod algorithm;
mod map;
mod render_utils;

use algorithm::{
    a_star::AStar, bfs::Bfs, dijkstra::Dijkstra, greedy_bfs::GreedyBfs, Algorithm, AlgorithmError,
};

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

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum MenuSelectionState {
        Bfs = 0,
        Dijkstra = 1,
        GreedyBfs = 2,
        AStar = 3,
    }

    impl MenuSelectionState {
        pub fn new() -> Self {
            Self::Bfs
        }

        pub fn next(self) -> Self {
            match self {
                Self::Bfs => Self::Dijkstra,
                Self::Dijkstra => Self::GreedyBfs,
                Self::GreedyBfs => Self::AStar,
                Self::AStar => Self::Bfs,
            }
        }

        pub fn prev(self) -> Self {
            match self {
                Self::Bfs => Self::AStar,
                Self::AStar => Self::GreedyBfs,
                Self::GreedyBfs => Self::Dijkstra,
                Self::Dijkstra => Self::Bfs,
            }
        }

        pub fn selected_algorithm_id(&self) -> usize {
            *self as usize
        }

        pub fn from_index(self, id: isize) -> Option<Self> {
            match id {
                0 => Some(Self::Bfs),
                1 => Some(Self::Dijkstra),
                2 => Some(Self::GreedyBfs),
                3 => Some(Self::AStar),
                _ => None,
            }
        }

        pub fn reset(self) -> Self {
            Self::Bfs
        }
    }
}

mod application {
    pub mod message {
        pub const WELCOME: &str = "..::R-PATH-FINDER::..\n\n - 1-click left mouse button sets start\n\n - 2-click left mouse button sets goal\n\n - right mouse button sets obstacle\n\n - 3-click left mouse button starts\n  the simulation\n\n - Esc - restart simulation";
        pub const SIMULATION_STARTS: &str = "Simulation Starts...";
        pub const APP_TITLE: &str = "R-PathFinder - Menu";
        pub const DONE: &str = "Done";
        pub const ALGORITHM_MENU_ITEMS: [&str; 4] = ["Bfs", "Dijkstra", "Greedy Bfs", "A*"];
    }
    #[derive(Debug, PartialEq)]
    pub enum Scene {
        Menu,
        Algorithm,
    }
}

pub struct App<'a> {
    window: PistonWindow,
    algorithms: [Box<dyn Algorithm>; 4],
    grid: Grid,
    mouse_action_fsm: fsm::MouseActionState,
    menu_fsm: fsm::MenuSelectionState,
    scene: application::Scene,
    output_log: &'a str,
}

impl Default for App<'_> {
    /// # default
    /// Create a new instance of application.
    ///
    /// The application creates all path-finding algorithms.
    fn default() -> Self {
        let window: PistonWindow =
            WindowSettings::new(application::message::APP_TITLE.to_string(), [700.0, 480.0])
                .build()
                .unwrap();

        let grid = Grid::new(0, 0, 400, 400, 20);

        let algorithms: [Box<dyn Algorithm>; application::message::ALGORITHM_MENU_ITEMS.len()] = [
            Box::new(Bfs::default()),
            Box::new(Dijkstra::default()),
            Box::new(GreedyBfs::default()),
            Box::new(AStar::default()),
        ];

        Self {
            window,
            algorithms,
            grid,
            mouse_action_fsm: fsm::MouseActionState::new(),
            menu_fsm: fsm::MenuSelectionState::new(),
            scene: application::Scene::Menu,
            output_log: application::message::WELCOME,
        }
    }
}

impl App<'_> {
    /// # run
    /// Run application/simulation
    pub fn run(&mut self) {
        let mut mouse_screen_position = [0.0, 0.0];
        let mut glyph = self.load_font_asset();
        let mut is_drawing_locked = false;

        while let Some(e) = self.window.next() {
            match self.scene {
                application::Scene::Menu => self.menu_scene_input_handling(&e),
                application::Scene::Algorithm => self.algorithm_scene_input_handling(
                    &mut mouse_screen_position,
                    &mut is_drawing_locked,
                    &e,
                ),
            }
            e.update(|args: &UpdateArgs| {
                self.update_simulation_state(args);
            });

            self.window.draw_2d(&e, |c, g, device| {
                clear(render_utils::color::BACKGROUND, g);

                if self.scene == application::Scene::Menu {
                    for (algorithm_id, menu_item) in application::message::ALGORITHM_MENU_ITEMS
                        .iter()
                        .enumerate()
                    {
                        let mut size = 32;

                        if algorithm_id == self.menu_fsm.selected_algorithm_id() {
                            size = 48;
                        }

                        render_utils::draw_text(
                            menu_item,
                            [270.0, 150.0 + 50.0 * (algorithm_id as f64)],
                            size,
                            render_utils::color::BLACK,
                            &mut glyph,
                            &c,
                            g,
                        );
                    }
                } else {
                    render_utils::draw_text(
                        self.output_log,
                        [410.0, 50.0],
                        16,
                        render_utils::color::BLACK,
                        &mut glyph,
                        &c,
                        g,
                    );

                    if self.algorithms[self.menu_fsm.selected_algorithm_id()].has_completed() {
                        render_utils::draw_text(
                            &self.algorithms[self.menu_fsm.selected_algorithm_id()]
                                .output_statistics(),
                            [410.0, 100.0],
                            16,
                            render_utils::color::BLACK,
                            &mut glyph,
                            &c,
                            g,
                        );
                    }

                    self.grid.render(&c, g);
                }

                glyph.factory.encoder.flush(device);
            });
        }
    }

    /// # skip_menu_and_run_algorithm
    /// Skip menu and just run current algorithm
    pub fn skip_menu_and_run_algorithm(&mut self, id: isize) -> Result<(), AlgorithmError> {
        let alg = self.menu_fsm.from_index(id);
        if alg.is_none() {
            return Err(AlgorithmError::AlgorithmDoesNotExist);
        }
        self.window
            .set_title(application::message::ALGORITHM_MENU_ITEMS[id as usize].to_string());
        self.menu_fsm = alg.unwrap();
        self.scene = application::Scene::Algorithm;

        Ok(())
    }

    fn menu_scene_input_handling(&mut self, e: &Event) {
        if let Some(Button::Keyboard(Key::Up)) = e.press_args() {
            self.menu_fsm = self.menu_fsm.prev();
        }

        if let Some(Button::Keyboard(Key::Down)) = e.press_args() {
            self.menu_fsm = self.menu_fsm.next();
        }

        if let Some(Button::Keyboard(Key::Return)) = e.press_args() {
            self.scene = application::Scene::Algorithm;
            self.window.set_title(
                application::message::ALGORITHM_MENU_ITEMS[self.menu_fsm.selected_algorithm_id()]
                    .to_string(),
            );
        }
    }

    fn algorithm_scene_input_handling(
        &mut self,
        mouse_screen_position: &mut [f64; 2],
        is_drawing_locked: &mut bool,
        e: &Event,
    ) {
        if let Some(pos) = e.mouse_cursor_args() {
            *mouse_screen_position = pos;
            if *is_drawing_locked {
                self.grid
                    .on_mouse_clicked(mouse_screen_position, Title::Obstacle);
            }
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            match button {
                MouseButton::Left => self.handle_mouse_action(*mouse_screen_position),
                MouseButton::Right => {
                    *is_drawing_locked = true;
                    self.grid
                        .on_mouse_clicked(mouse_screen_position, Title::Obstacle);
                }
                _ => (),
            }
        }

        if let Some(Button::Mouse(button)) = e.release_args() {
            if button == MouseButton::Right {
                *is_drawing_locked = false;
            }
        }

        if let Some(Button::Keyboard(Key::Escape)) = e.press_args() {
            self.reset_simulation();
        }
    }

    fn load_font_asset(&mut self) -> Glyphs {
        let font_source_path = std::env::current_dir()
            .unwrap()
            .join("assets/fonts/Roboto-Bold.ttf");
        self.window.load_font(font_source_path).unwrap()
    }

    fn update_simulation_state(&mut self, args: &UpdateArgs) {
        if self.algorithms[self.menu_fsm.selected_algorithm_id()].has_completed() {
            self.output_log = application::message::DONE;
            return;
        }
        self.algorithms[self.menu_fsm.selected_algorithm_id()]
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
                let s =
                    self.algorithms[self.menu_fsm.selected_algorithm_id()].start(&mut self.grid);
                self.handle_algorithm_error(s);
                self.mouse_action_fsm = self.mouse_action_fsm.next();
            }
            fsm::MouseActionState::EndSimulation => {}
        }
    }

    fn reset_simulation(&mut self) {
        self.output_log = application::message::WELCOME;
        self.mouse_action_fsm = self.mouse_action_fsm.reset();
        self.algorithms[self.menu_fsm.selected_algorithm_id()].reset(&mut self.grid);

        self.menu_fsm = self.menu_fsm.reset();
        self.scene = application::Scene::Menu;
        self.window
            .set_title(application::message::APP_TITLE.to_string());
    }

    fn handle_algorithm_error(&mut self, status: Result<(), AlgorithmError>) {
        match status {
            Err(AlgorithmError::InvalidInputData) => {
                println!("User did not set the start or end point")
            }
            Err(_) => todo!(),
            Ok(_) => {
                self.output_log = application::message::SIMULATION_STARTS;
            }
        }
    }
}
