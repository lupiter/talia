use crate::game::types::*;
use gb_rs::prelude::*;

pub struct Player {
    pub position: Position,
    pub direction: Direction,
    pub sprite_id: u8,
    pub animation_frame: u8,
    pub animation_timer: u8,
}

impl Player {
    pub fn new(start_pos: Position, sprite_id: u8) -> Self {
        Player {
            position: start_pos,
            direction: Direction::Down,
            sprite_id,
            animation_frame: 0,
            animation_timer: 0,
        }
    }

    pub fn update(&mut self, gb: &GameBoy) {
        // Update animation
        self.animation_timer = self.animation_timer.wrapping_add(1);
        if self.animation_timer == 0 {
            self.animation_frame = (self.animation_frame + 1) % 2;
        }

        // Handle movement
        let mut new_pos = self.position;
        if gb.input.is_pressed(Button::Up) {
            new_pos.y = new_pos.y.saturating_sub(1);
            self.direction = Direction::Up;
        }
        if gb.input.is_pressed(Button::Down) {
            new_pos.y = new_pos.y.saturating_add(1);
            self.direction = Direction::Down;
        }
        if gb.input.is_pressed(Button::Left) {
            new_pos.x = new_pos.x.saturating_sub(1);
            self.direction = Direction::Left;
        }
        if gb.input.is_pressed(Button::Right) {
            new_pos.x = new_pos.x.saturating_add(1);
            self.direction = Direction::Right;
        }

        self.position = new_pos;
    }

    pub fn draw(&self, gb: &mut GameBoy) {
        // Calculate sprite offset based on direction and animation
        let sprite_offset = match self.direction {
            Direction::Up => 0,
            Direction::Down => 4,
            Direction::Left => 8,
            Direction::Right => 12,
        } + self.animation_frame;
        
        gb.display.draw_sprite(
            self.position.x,
            self.position.y,
            self.sprite_id + sprite_offset,
        );
    }

    pub fn try_interact(&self, map: &Map) -> Option<InteractionType> {
        let interact_pos = match self.direction {
            Direction::Up => Position {
                x: self.position.x,
                y: self.position.y.saturating_sub(1),
            },
            Direction::Down => Position {
                x: self.position.x,
                y: self.position.y.saturating_add(1),
            },
            Direction::Left => Position {
                x: self.position.x.saturating_sub(1),
                y: self.position.y,
            },
            Direction::Right => Position {
                x: self.position.x.saturating_add(1),
                y: self.position.y,
            },
        };
        
        map.handle_interaction(interact_pos)
    }
} 