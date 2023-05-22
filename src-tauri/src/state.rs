use std::sync::Mutex;

use rand::{seq::IteratorRandom, thread_rng, Rng};
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct GameState {
    grid: Mutex<Vec<u16>>,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            grid: Mutex::new(initial_random_state()),
        }
    }

    pub fn up(&self) -> Vec<u16> {
        *self.grid.lock().unwrap() = vec![1, 2];
        self.grid.lock().unwrap().clone()
    }

    pub fn down(&self) -> Vec<u16> {
        self.grid.lock().unwrap().clone()
    }

    pub fn left(&self) -> Vec<u16> {
        self.grid.lock().unwrap().clone()
    }

    pub fn right(&self) -> Vec<u16> {
        self.grid.lock().unwrap().clone()
    }

    pub fn current(&self) -> Vec<u16> {
        self.grid.lock().unwrap().clone()
    }
}

fn initial_random_state() -> Vec<u16> {
    let mut grid_cells = Vec::from([0; 16]);
    let mut rng = thread_rng();
    let range: Vec<u8> = (0..=15).collect();
    let random_indexes = range.iter().choose_multiple(&mut rng, 2);

    for index in random_indexes.into_iter() {
        let rand_value = rng.gen_range(1..=2) * 2;
        grid_cells[*index as usize] = rand_value;
    }
    grid_cells
}

fn process_up(state: &GameState) -> Option<String> {
    // let data = &mut state.grid.lock().unwrap();
    // for (i, cell) in data.iter().enumerate() {
    //     if *cell == 0 {
    //         *cell = 3;
    //     }
    // }
    todo!()
}
