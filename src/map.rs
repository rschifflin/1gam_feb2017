use facing::Facing;

#[derive(Debug, Deserialize, PartialEq)]
pub enum LayerType {
  #[serde(rename = "tilelayer")]
  TileLayer,
  #[serde(rename = "objectgroup")]
  ObjectGroup,
  #[serde(rename = "imagelayer")]
  ImageLayer
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum ObjectType {
  Bird,
  LeftRight,
  UpDown,
  Arc,
  Pirate
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ObjectProperties {
  // Bird properties
  pub requisite_progress: Option<u32>,
  pub reward_progress: Option<u32>,
  //

  // Enemy Properties
  pub speed: Option<f64>,
  pub radius: Option<f64>,
  pub angle: Option<f64>,
  pub frequency: Option<f64>,
  pub flip: Option<bool>
}

#[derive(Debug, Deserialize)]
pub struct Object {
  pub name: String,
  #[serde(rename = "type")]
  pub object_type: ObjectType,
  pub x: usize,
  pub y: usize,
  pub width: usize,
  pub height: usize,
  pub properties: ObjectProperties
}

#[derive(Debug, Deserialize)]
pub struct TileSet {
  pub image: String,
  pub name: String,
  pub tilewidth: usize,
  pub tileheight: usize
}

#[derive(Debug, Deserialize)]
pub struct Map {
  pub height: usize,
  pub layers: Vec<Layer>,
  pub tileheight: usize,
  pub tilesets: Vec<TileSet>,
  pub tilewidth: usize,
  pub version: usize,
  pub width: usize
}

#[derive(Debug, Deserialize)]
pub struct Layer {
  pub height: usize,
  #[serde(rename = "type")]
  pub layer_type: LayerType,
  pub data: Option<Vec<usize>>,
  pub objects: Option<Vec<Object>>,
  pub width: usize
}

#[derive(Debug, PartialEq)]
pub enum Tile {
  Blank,
  HeroStart,
  Floor,
  Checkpoint,
  Portal,
  Spike(Facing)
}

pub fn tile_from_id(id: usize) -> Tile {
  match id {
    1 => Tile::Floor,
    2 => Tile::Checkpoint,
    5 => Tile::Portal,
    10 => Tile::HeroStart,
    20 => Tile::Spike(Facing::Up),
    29 => Tile::Spike(Facing::Down),
    38 => Tile::Spike(Facing::Left),
    47 => Tile::Spike(Facing::Right),
    _ => Tile::Blank
  }
}
