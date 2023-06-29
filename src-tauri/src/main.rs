#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::sync::Mutex;
use commands::{
    login::login,
    save_post::save_post
};

static URL: Mutex<String> = Mutex::new(String::new());
static HASH: Mutex<String> = Mutex::new(String::new());

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_post, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
