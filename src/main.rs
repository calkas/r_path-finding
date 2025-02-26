use r_path_finder::{algorithm, App};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let bfs = algorithm::bfs::Bfs::default();

    let algorithm: Rc<RefCell<dyn algorithm::Algorithm>> = Rc::new(RefCell::new(bfs));

    let mut app = App::new(algorithm.clone());

    app.run();
}
