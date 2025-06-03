extern crate r_path_finder;

use r_path_finder::algorithm::{greedy_bfs::GreedyBfs, Algorithm};
use r_path_finder::App;

fn main() {
    println!("====== Example ======");
    println!("..::Greedy BFS:...");
    let greedy_bfs: Box<dyn Algorithm> = Box::new(GreedyBfs::default());

    let mut app = App::new(greedy_bfs);

    app.run();
    println!("======== DONE =======");
}
