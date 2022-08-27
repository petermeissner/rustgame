

#[derive(Default, Debug)]
pub struct Cell {
  pub x: u32,
  pub y: u32
}

#[derive(Default, Debug)]
pub struct Position {
  pub x: u32,
  pub y: u32
}



#[derive(Default)]
pub struct Game {
    pub map: Vec<Vec<Cell>>,
    pub player: Player,
}



#[derive(Default)]
pub struct Player {
    pub position: Position
}
