use std::collections::HashMap;

fn main() {
    let map_string: &str = "# ^
  # ^ ";

    fn map_char_to_path(s: &str) -> &'static str {
        let map_char_texture: HashMap<&str, &str> = HashMap::from([
            ("^", "collection/tree_01.png"),
            ("#", "collection/wall_01.png"),
            (" ", "collection/floor_01.png"),
            ("\n", ""),
        ]);

        return map_char_texture[s];
    }
    
    for c in map_string.chars() {
      print!("\n{c}");
      let cc = map_char_to_path(&c.to_string());
      print!(" {cc}")
    }
}
