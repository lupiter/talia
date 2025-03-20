use crate::game::types::*;
use gb_rs::prelude::*;

pub struct Map {
    pub name: &'static str,
    pub width: u8,
    pub height: u8,
    pub tiles: Vec<MapObject>,
    pub entities: Vec<Entity>,
    pub background_tiles: &'static [u8],
    pub collision_map: &'static [bool],
}

impl Map {
    pub fn new(
        name: &'static str,
        width: u8,
        height: u8,
        background_tiles: &'static [u8],
        collision_map: &'static [bool],
    ) -> Self {
        Map {
            name,
            width,
            height,
            tiles: Vec::new(),
            entities: Vec::new(),
            background_tiles,
            collision_map,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn add_tile(&mut self, tile: MapObject) {
        self.tiles.push(tile);
    }

    pub fn is_walkable(&self, pos: Position) -> bool {
        if pos.x >= self.width || pos.y >= self.height {
            return false;
        }
        let index = (pos.y as usize * self.width as usize) + pos.x as usize;
        !self.collision_map[index]
    }

    pub fn get_tile_at(&self, pos: Position) -> Option<&MapObject> {
        self.tiles.iter().find(|t| t.position == pos)
    }

    pub fn get_entity_at(&self, pos: Position) -> Option<&Entity> {
        self.entities.iter().find(|e| e.position == pos)
    }

    pub fn draw(&self, gb: &mut GameBoy) {
        // Draw background tiles
        gb.display.load_tiles(0, self.background_tiles);
        
        // Draw the map
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y as usize * self.width as usize) + x as usize;
                gb.display.draw_tile(x, y, index as u8);
            }
        }

        // Draw entities
        for entity in &self.entities {
            gb.display.draw_sprite(entity.position.x, entity.position.y, entity.sprite_id);
        }
    }

    pub fn handle_interaction(&self, pos: Position) -> Option<InteractionType> {
        if let Some(tile) = self.get_tile_at(pos) {
            tile.interaction_type
        } else {
            None
        }
    }
} 