# Talia - A GameBoy Game in Rust

This is a simple GameBoy game written in Rust using the `gb-rs` framework.

## Prerequisites

- Rust (latest stable version)
- GameBoy emulator (recommended: BGB, SameBoy, or mGBA)

## Building

To build the project:

```bash
cargo build --release
```

The compiled ROM will be available in `target/release/talia.gb`

## Running

You can run the ROM using any GameBoy emulator that supports .gb files. The program will display "Hello World!" on the screen.

## Development

This is a basic GameBoy project that demonstrates:
- Setting up a GameBoy project in Rust
- Displaying text on the GameBoy screen
- Basic game loop structure