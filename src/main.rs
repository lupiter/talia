#![no_std]
#![no_main]

use gb_rs::prelude::*;
mod game;
use game::{Map, Player, Position, Direction};

// Include the generated tile data
mod generated;
use generated::TITLE_TILES;

// Game states
#[derive(PartialEq)]
enum GameState {
    Title,
    Playing,
}

// Game data
struct GameData {
    current_map: Map,
    player: Player,
}

#[no_mangle]
pub fn main() -> ! {
    // Initialize the GameBoy
    let mut gb = GameBoy::new();
    let mut game_state = GameState::Title;
    
    // Initialize game data
    let mut game_data = GameData {
        current_map: Map::new(
            "Starting Area",
            20,
            18,
            &[], // We'll add background tiles later
            &[], // We'll add collision map later
        ),
        player: Player::new(Position { x: 10, y: 10 }, 0),
    };
    
    // Set up the display
    gb.display.set_mode(DisplayMode::Bitmap);
    
    // Main game loop
    loop {
        match game_state {
            GameState::Title => {
                // Clear the screen
                gb.display.clear();
                
                // Load and display title screen tiles
                gb.display.load_tiles(0, TITLE_TILES);
                
                // Draw the title screen (assuming it's 20x18 tiles)
                for y in 0..18 {
                    for x in 0..20 {
                        gb.display.draw_tile(x, y, (y * 20 + x) as u8);
                    }
                }
                
                // Draw text overlay
                gb.display.set_mode(DisplayMode::Text);
                gb.display.print_at(3, 16, "Press START");
                
                // Check for START button press
                if gb.input.is_pressed(Button::Start) {
                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // Update game state
                game_data.player.update(&gb);
                
                // Handle interactions
                if gb.input.is_pressed(Button::A) {
                    if let Some(interaction) = game_data.player.try_interact(&game_data.current_map) {
                        match interaction {
                            InteractionType::Door(map_name) => {
                                // Handle map transition
                                // TODO: Implement map transitions
                            }
                            InteractionType::NPC(npc_id) => {
                                // Handle NPC interaction
                                // TODO: Implement NPC interactions
                            }
                            InteractionType::Item(item_id) => {
                                // Handle item collection
                                // TODO: Implement item collection
                            }
                            InteractionType::Sign(text) => {
                                // Handle sign reading
                                // TODO: Implement sign reading
                            }
                        }
                    }
                }
                
                // Draw game state
                game_data.current_map.draw(&mut gb);
                game_data.player.draw(&mut gb);
                
                // Check for START button press to return to title
                if gb.input.is_pressed(Button::Start) {
                    game_state = GameState::Title;
                }
            }
        }
        
        // Update the display
        gb.display.update();
        
        // Wait for V-Blank
        gb.wait_vblank();
    }
} 