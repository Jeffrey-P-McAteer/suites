
mod gui;
mod structs;

fn main() {
  if let Err(e) = gui::run_main() {
    eprintln!("Error in gui::run_main: {:?}", e);
  }
}



