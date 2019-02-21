extern crate nalgebra;

use nalgebra::{Matrix2, Vector2}; //, Vector3, Matrix3};
use std::collections::HashMap;

pub struct Simulator {
    states: Vec<Matrix2<i32>>,
    pub board: HashMap<Vector2<i32>, usize>,
    pub ant_position: Vector2<i32>,
    pub ant_direction: Vector2<i32>,
}

impl Simulator {
    pub fn new() -> Result<Self, &'static str> {
        let mut states: Vec<Matrix2<i32>> = Vec::new();
        states.push(Matrix2::new(0, -1, 1, 0));
        states.push(Matrix2::new(0, 1, -1, 0));

        Ok(Simulator {
            states: states.clone(),
            board: HashMap::new(),
            ant_position: Vector2::new(0, 0),   // Ant is at origin
            ant_direction: Vector2::new(-1, 0), // facing left
        })
    }

    pub fn simulate(&mut self) {
        // Get the color of the square under the ant. Default to white
        let square_color = self.board.get(&self.ant_position).cloned().unwrap_or(0);
        // Rotate by the state of square.
        self.ant_direction = self.states[square_color] * self.ant_direction;
        // Advance the state of the square by 1, possible wrap to back to 0
        self.board
            .insert(self.ant_position, (square_color + 1) % self.states.len());
        self.ant_position += self.ant_direction; // Move the ant by its direction
    }
}

fn main() {
    let max_steps = 1000000;

    let mut sim = Simulator::new().unwrap();
    for _ in 0..max_steps {
        sim.simulate();
    }
}
