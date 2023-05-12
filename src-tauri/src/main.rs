// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #![allow(unused)]

use std::sync::Mutex;

use rand::{seq::IteratorRandom, thread_rng, Rng};

fn main() {
    let state = GameState::default();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .manage(state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct GameState {
    grid_cell: Mutex<Vec<u16>>,
}
impl GameState {
    fn default() -> Self {
        Self {
            grid_cell: Mutex::new(initial_random_state()),
        }
    }
}

#[tauri::command]
fn greet(key: &str) -> String {
    match key {
        "ArrowLeft" => {
            todo!()
        }
        "ArrowRight" => {
            todo!()
        }
        "ArrowUp" => {
            todo!()
        }
        "ArrowDown" => {
            todo!()
        }
        _ => todo!(),
    }
}

fn initial_random_state() -> Vec<u16> {
    let mut grid_cells = Vec::from([0 as u16; 16]);
    let mut rng = thread_rng();
    let range: Vec<u8> = (0..=15).collect();
    let random_indexes = range.iter().choose_multiple(&mut rng, 2);

    for index in random_indexes.into_iter() {
        let rand_value = rng.gen_range(1..=2) * 2;
        grid_cells[*index as usize] = rand_value;
    }
    grid_cells
}
