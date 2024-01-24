// Import necessary modules and external functions
use piston_window::*;
use piston_window::types::{Color, Width};
use rand::{thread_rng, Rng};

// Import Snake struct and Direction enum from the snake module
use crate::snake::{Direction, Snake};

// Import external draw functions from the draw module
use crate::draw::{self, draw_block, draw_rectangle};

// Define constants for colors and game parameters
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

// Define the Game struct to represent the game state
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

impl Game {
    // Constructor to create a new Game instance with an initial snake and food position
    pub fn new(Width: i32, Height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width: Width,
            height: Height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    // Handle key presses to change the direction of the snake
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        // Prevent the snake from immediately reversing direction
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        // Update the snake's direction
        self.update_snake(dir);
    }

    // Draw the game elements on the screen
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Draw the snake
        self.snake.draw(con, g);

        // Draw the food if it exists
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Draw the game board borders
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        // Draw a game over overlay if the game is over
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    // Update the game state based on elapsed time
    pub fn update(&mut self, delta_time: f64) {
        // Increment the waiting time
        self.waiting_time += delta_time;

        // If the game is over, wait for a certain period before restarting
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        // If the food doesn't exist, add a new food block
        if !self.food_exists {
            self.add_food();
        }

        // Move the snake forward at regular intervals
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // Check if the snake has eaten the food and update the game state accordingly
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();

        // Check if the snake's head is on the same position as the food
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    // Check if the snake is still alive based on its current and next position
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head(dir);

        // Check if the snake hit a border
        if next_x <= 0 || next_y <= 0 || next_x >= self.width - 1 || next_y >= self.height - 1 {
            return false;
        }

        // Check if the snake hit itself
        !self.snake.overlap_tail(next_x, next_y)
    }

    // Add a new food block to the game at a random position
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        // Generate random coordinates for the new food block
        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.width - 1);

        // Ensure that the new food block does not overlap with the snake's body
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.width - 1);
        }

        // Set the coordinates of the new food block and mark its existence
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    // Update the snake's position based on the specified direction
    fn update_snake(&mut self, dir: Option<Direction>) {
        // Check if the snake is still alive before updating its position
        if self.check_if_snake_alive(dir) {
            // Move the snake forward and check for eating
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            // If the snake is not alive, set the game over flag
            self.game_over = true;
        }

        // Reset the waiting time for the next movement
        self.waiting_time = 0.0;
    }

    // Restart the game by resetting the snake, food, and game over state
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
        
}