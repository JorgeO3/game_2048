// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
mod state;

use state::GameState;
use tauri::State;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .manage(GameState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(key: &str, state: State<GameState>) -> Vec<u16> {
    match key {
        "ArrowUp" => state.up(),
        "ArrowDown" => state.down(),
        "ArrowLeft" => state.left(),
        "ArrowRight" => state.right(),
        _ => state.current(),
    }
}
