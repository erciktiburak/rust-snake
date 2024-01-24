// Import necessary modules and external functions
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

// Import the external draw_block function from the draw module
use draw::draw_block;

// Define the color for the snake
const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

// Enum to represent the possible directions of the snake
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Method to get the opposite direction
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Struct to represent a block in the snake's body
struct Block {
    x: i32,
    y: i32,
}

// Struct to represent the Snake
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    // Constructor to create a new Snake with an initial position
    pub fn new(x: i32, y: i32) -> Snake {
        // Create a new linked list representing the snake's body
        let mut body: LinkedList<Block> = LinkedList::new();

        // Add three blocks to the body, creating an initial snake
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        // Return the initialized Snake struct
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Iterate through each block in the snake's body
        for block in &self.body {
            // Draw the block
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        // Get the position of the first block in the snake's body
        let head_block = self.body.front().unwrap();

        // Return the position of the block
        (head_block.x, head_block.y)
    }

    // Method to change the direction of the snake
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            // If a direction was specified, change the snake's direction
            Some(d) => self.direction = d,
            // Otherwise, keep the current direction
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        // Create a new block in the direction the snake is moving
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
    }
}
