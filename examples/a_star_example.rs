extern crate r_path_finder;

use r_path_finder::algorithm::{a_star::AStar, Algorithm};
use r_path_finder::App;

fn main() {
    println!("====== Example ======");
    println!("..::A* Algorithm:...");
    let a_star: Box<dyn Algorithm> = Box::new(AStar::default());

    let mut app = App::new(a_star);

    app.run();
    println!("======== DONE =======");
}
