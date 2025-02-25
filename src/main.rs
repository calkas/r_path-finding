extern crate piston_window;

mod algorithm;
mod map;

use crate::map::grid::Grid;

use crate::algorithm::Algorithm;
use algorithm::bfs::Bfs;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Path Finding", [640.0, 480.0])
        .build()
        .unwrap();

    let mut bfs = Bfs::default();
    let mut grid = Grid::new(0, 0, 400, 400, 20);
    let mut left_clicked_times = 0;
    let mut mouse_pos = [0.0, 0.0];

    while let Some(e) = window.next() {
        if let Some(pos) = e.mouse_cursor_args() {
            mouse_pos = pos;
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            match button {
                MouseButton::Left => {
                    if left_clicked_times == 0 {
                        grid.on_mouse_clicked(&mouse_pos, map::grid::Title::Start);
                    } else if left_clicked_times == 1 {
                        grid.on_mouse_clicked(&mouse_pos, map::grid::Title::End);
                    } else if left_clicked_times == 2 {
                        bfs.start(&mut grid);
                    }
                    left_clicked_times = left_clicked_times + 1;
                }
                MouseButton::Right => {
                    grid.on_mouse_clicked(&mouse_pos, map::grid::Title::Obstacle);
                }
                _ => (),
            }
        }

        if let Some(Button::Keyboard(Key::Escape)) = e.press_args() {
            println!("Reset");
            left_clicked_times = 0;
            bfs.reset(&mut grid);
        }

        e.update(|args| {
            bfs.update(&mut grid, args.dt);
        });

        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            grid.render(&c, g);
        });
    }
}
