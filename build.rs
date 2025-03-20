use std::path::Path;
use gb_tools::image_converter::{convert_image_to_tiles, TileSet};

fn main() {
    // Convert title screen image
    let title_image_path = Path::new("assets/title.png");
    if title_image_path.exists() {
        let tiles = convert_image_to_tiles(title_image_path, TileSet::GB);
        let output_path = Path::new("src/generated/title_tiles.rs");
        
        // Create the generated directory if it doesn't exist
        std::fs::create_dir_all("src/generated").unwrap();
        
        // Generate the Rust code containing the tile data
        let mut output = String::new();
        output.push_str("pub const TITLE_TILES: &[u8] = &[\n");
        for tile in tiles.iter() {
            output.push_str("    ");
            for byte in tile.iter() {
                output.push_str(&format!("0x{:02X}, ", byte));
            }
            output.push('\n');
        }
        output.push_str("];\n");
        
        std::fs::write(output_path, output).unwrap();
    }

    // Convert bedroom tiles
    let bedroom_tiles_path = Path::new("assets/tiles/bedroom.png");
    if bedroom_tiles_path.exists() {
        let tiles = convert_image_to_tiles(bedroom_tiles_path, TileSet::GB);
        let output_path = Path::new("src/generated/bedroom_tiles.rs");
        
        // Create the generated directory if it doesn't exist
        std::fs::create_dir_all("src/generated").unwrap();
        
        // Generate the Rust code containing the tile data
        let mut output = String::new();
        output.push_str("pub const BEDROOM_TILES: &[u8] = &[\n");
        for tile in tiles.iter() {
            output.push_str("    ");
            for byte in tile.iter() {
                output.push_str(&format!("0x{:02X}, ", byte));
            }
            output.push('\n');
        }
        output.push_str("];\n");
        
        std::fs::write(output_path, output).unwrap();
    }
} 