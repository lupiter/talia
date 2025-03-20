use crate::game::map::Map;
use crate::game::types::{MapObject, Position, TileType, InteractionType};
use crate::generated::BEDROOM_TILES;

// Collision map (true = collision, false = walkable)
pub const BEDROOM_COLLISION: &[bool] = &[
    // 20x18 grid
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
];

pub fn create_bedroom() -> Map {
    let mut map = Map::new("Bedroom", 20, 18, BEDROOM_TILES, BEDROOM_COLLISION);
    
    // Add walls
    for x in 0..20 {
        map.add_tile(MapObject {
            position: Position { x, y: 0 },
            tile_type: TileType::Wall,
            interaction_type: None,
        });
        map.add_tile(MapObject {
            position: Position { x, y: 17 },
            tile_type: TileType::Wall,
            interaction_type: None,
        });
    }
    for y in 0..18 {
        map.add_tile(MapObject {
            position: Position { x: 0, y },
            tile_type: TileType::Wall,
            interaction_type: None,
        });
        map.add_tile(MapObject {
            position: Position { x: 19, y },
            tile_type: TileType::Wall,
            interaction_type: None,
        });
    }
    
    // Add door
    map.add_tile(MapObject {
        position: Position { x: 19, y: 8 },
        tile_type: TileType::Door,
        interaction_type: Some(InteractionType::Door("Hallway".to_string())),
    });
    
    map
} 