extern crate piston_window;

mod map;

use crate::map::grid::Grid;

use piston_window::{types::Color, *};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Path Finding", [640.0, 480.0])
        .build()
        .unwrap();

    let mut grid = Grid::new(0, 0, 400, 400, 20);
    let mut left_clicked_times = 0;

    let mut mouse_pos = [0.0, 0.0];
    while let Some(e) = window.next() {
        if let Some(pos) = e.mouse_cursor_args() {
            mouse_pos = pos;
            //println!("mouse pos = {:?}", mouse_pos);
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            match button {
                MouseButton::Left => {
                    if left_clicked_times == 0 {
                        grid.on_mouse_clicked(&mouse_pos, map::grid::Title::Start);
                    } else if left_clicked_times == 1 {
                        grid.on_mouse_clicked(&mouse_pos, map::grid::Title::End);
                    } else {
                    }
                    left_clicked_times = left_clicked_times + 1;
                }
                MouseButton::Right => {
                    grid.on_mouse_clicked(&mouse_pos, map::grid::Title::Obstacle);
                }
                _ => (),
            }
        }
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            grid.render(&c, g);
        });
    }
}

// fn main() {
//     let mut grid = Grid::new(200, 200, 20);
//     let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
//         .exit_on_esc(true)
//         .build()
//         .unwrap();

//     while let Some(e) = window.next() {
//         window.draw_2d(&e, |c, g, _device| {
//             clear([1.0; 4], g);
//             rectangle(
//                 [1.0, 0.0, 0.0, 1.0], // red
//                 [0.0, 0.0, 100.0, 100.0],
//                 c.transform,
//                 g,
//             );
//         });
//     }
// }
