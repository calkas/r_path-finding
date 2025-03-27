extern crate r_path_finder;

use r_path_finder::algorithm::{bfs::Bfs, Algorithm};
use r_path_finder::App;

fn main() {
    println!("====== Example ======");
    println!("..::BFS Algorithm:...");
    let bfs: Box<dyn Algorithm> = Box::new(Bfs::default());

    let mut app = App::new(bfs);

    app.run();
}
