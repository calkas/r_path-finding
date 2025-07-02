extern crate r_path_finder;
use r_path_finder::App;

fn main() {
    println!("====== Example ======");
    println!("..::A* Algorithm:...");
    let mut app = App::default();
    if app.skip_menu_and_run_algorithm(3).is_ok() {
        app.run();
    }
    println!("======== DONE =======");
}
