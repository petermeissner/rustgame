use std::fs;

fn main() {
    
  fn read_text(path: &str) -> String {
    let data = fs::read_to_string(path).expect("Unable to read file");
    return data;
  }
  println!("{}", read_text("assets/maps/examples/001.txt"));
}
