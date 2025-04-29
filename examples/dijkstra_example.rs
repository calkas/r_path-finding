extern crate r_path_finder;

use r_path_finder::algorithm::{dijkstra::Dijkstra, Algorithm};
use r_path_finder::App;

fn main() {
    println!("====== Example ======");
    println!("..::Dijkstra Algorithm:...");
    let dijkstra: Box<dyn Algorithm> = Box::new(Dijkstra::default());

    let mut app = App::new(dijkstra);

    app.run();
    println!("======== DONE =======");
}
