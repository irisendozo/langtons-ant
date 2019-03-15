extern crate nalgebra;

use nalgebra::Vector2;
use std::collections::HashMap;
use std::env;

pub struct Ant {
    pub board: HashMap<Vector2<i32>, bool>,
    pub position: Vector2<i32>,
    pub direction: i8,
}

impl Ant {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Ant {
            board: HashMap::new(),
            position: Vector2::new(0, 0),
            // Up: 0, Right: 1, Down: 2, Left: 3
            direction: 3,
        })
    }

    pub fn simulate(&mut self) {
        let color = self.board.get(&self.position).cloned().unwrap_or(false);

        match color {
            // Black is false
            false => {
                if self.direction == 3 {
                    self.direction = 0
                } else {
                    self.direction = self.direction + 1;
                }
            }
            // White is true
            true => {
                if self.direction == 0 {
                    self.direction = 3
                } else {
                    self.direction = self.direction - 1;
                }
            }
        }

        // Invert the thing first before changing position
        self.board.insert(self.position, !color);

        match self.direction {
            0 => self.position.y = self.position.y + 1,
            1 => self.position.x = self.position.x + 1,
            2 => self.position.y = self.position.y - 1,
            3 => self.position.x = self.position.x - 1,
            _ => print!("some error"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = &args[1];

    match flag.as_ref() {
        "0" => {
            let max_steps = 1000000000;

            let mut ant = Ant::new().unwrap();
            for _ in 0..max_steps {
                ant.simulate();
            }

            print!("x: {}, y: {}", ant.position.x, ant.position.y)
        }
        "1" => {
            let max_steps = 11000;

            let mut ant = Ant::new().unwrap();
            for _ in 0..max_steps {
                ant.simulate();
            }
        }
        _ => print!("some error"),
    }
}
