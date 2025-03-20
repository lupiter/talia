#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Door,
    Grass,
    Water,
    Object(u8), // Object type ID
}

#[derive(Copy, Clone, PartialEq)]
pub enum EntityType {
    Player,
    NPC(u8), // NPC type ID
    Item(u8), // Item type ID
}

pub struct Entity {
    pub position: Position,
    pub entity_type: EntityType,
    pub sprite_id: u8,
}

pub struct MapObject {
    pub position: Position,
    pub tile_type: TileType,
    pub interaction_type: Option<InteractionType>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum InteractionType {
    Door(String), // Map name to transition to
    NPC(u8),      // NPC ID to interact with
    Item(u8),     // Item ID to collect
    Sign(String), // Text to display
} 