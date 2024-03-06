// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;
mod model;
mod service;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            controller::get_common_station,
            controller::get_all_station,
            controller::get_left_ticket
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
