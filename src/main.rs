use r_path_finder::algorithm::{bfs::Bfs, Algorithm};
use r_path_finder::App;

fn main() {
    let bfs: Box<dyn Algorithm> = Box::new(Bfs::default());

    let mut app = App::new(bfs);

    app.run();
}
