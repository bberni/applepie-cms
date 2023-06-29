#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::sync::Mutex;
use commands::{
    login::login,
    save_post::save_post,
    delete_post::delete_post,
    post_list::post_list,
};

static URL: Mutex<String> = Mutex::new(String::new());
static HASH: Mutex<String> = Mutex::new(String::new());

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_post, delete_post, post_list, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
