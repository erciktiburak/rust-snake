// Import necessary modules and external functions
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

// Import the external draw_block function from the draw module
use crate::draw::draw_block;

// Define the color for the snake
const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

// Enum to represent the possible directions of the snake
#[derive(Copy, Clone, PartialEq)]
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
#[derive(Debug, Clone)]
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

    // Method to draw the snake on the screen
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Iterate through each block in the snake's body and draw it
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    // Method to get the position of the snake's head
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    // Method to move the snake forward in the specified direction
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // Change the snake's direction if a new direction is specified
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        // Create a new block in the direction the snake is moving
        let new_block = match self.direction {
            Direction::Up => Block { x: last_x, y: last_y - 1 },
            Direction::Down => Block { x: last_x, y: last_y + 1 },
            Direction::Left => Block { x: last_x - 1, y: last_y },
            Direction::Right => Block { x: last_x + 1, y: last_y },
        };

        // Push the new block to the front of the snake's body
        self.body.push_front(new_block);

        // Pop the last block from the back of the snake's body and store it as the tail
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    // Method to get the current direction of the snake
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // Method to get the next position of the snake's head in the specified direction
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        // Determine the direction in which the snake is moving
        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        // Calculate the next position of the snake's head based on the current direction
        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    // Method to restore the tail of the snake
    pub fn restore_tail(&mut self) {
        // Clone the tail block and push it to the back of the snake's body
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    // Method to check if a given position overlaps with the snake's tail
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        // Iterate through each block in the snake's body
        for block in &self.body {
            // Check if the given position matches the position of any block in the body
            if x == block.x && y == block.y {
                return true;
            }
        }
        // If no overlap is found, return false
        false
    }
}
