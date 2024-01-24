// Import necessary modules from the piston_window crate
use piston_window::{color, rectangle, Context, G2d};
use piston_window::types::Color;

// Define the size of each block in the game grid
const BLOCK_SIZE: f64 = 25.0;

// Convert game coordinates to GUI coordinates
pub fn to_coord(game_coord: i32) -> f64 {
    // Multiply the game coordinate by the block size to get the GUI coordinate
    (game_coord as f64) * BLOCK_SIZE
}

// Draw a colored block at a specific position on the screen
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    // Convert game coordinates to GUI coordinates
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    // Draw a rectangle with the specified color at the GUI coordinates
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    // Convert game coordinates to GUI coordinates
    let x = to_coord(x);
    let y = to_coord(y);

    // Draw a rectangle with the specified color at the GUI coordinates
    rectangle(
        color,
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        con.transform,
        g,
    );
}
