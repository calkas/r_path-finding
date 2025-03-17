extern crate r_path_finder;
use r_path_finder::{algorithm, App};
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    println!("====== Example ======");
    println!("..::BFS Algorithm:...");
    let bfs = algorithm::bfs::Bfs::default();

    let algorithm: Rc<RefCell<dyn algorithm::Algorithm>> = Rc::new(RefCell::new(bfs));

    let mut app = App::new(algorithm.clone());

    app.run();
}
