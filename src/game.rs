use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    // The food is represented by an Option<Block> because it can be absent from the game
    food_exists: bool,
    food_x: i32,
    food_y: i32,

    // The width and height of the game board in blocks
    width: i32,
    height: i32,

    // The state of the game over flag
    game_over: bool,

    // The time elapsed since the start of the game
    waiting_time: f64,
}