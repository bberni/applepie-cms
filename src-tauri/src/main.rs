#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hex;
use reqwest;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{fs::File, io::Write};

static mut URL: String = String::new();
static mut HASH: String = String::new();

#[derive(Serialize)]
struct Password {
    hash: String,
}

#[derive(Serialize)]
struct Post {
    title: String,
    img: String,
    content: String
}

#[tauri::command]
fn save_post(title: &str, img_path: &str, content: &str) -> String {
    let mut post_file = match File::create(format!("{}.txt", title)) {
        Ok(f) => f,
        Err(e) => return format!("Error: {}", e).to_owned(),
    };
    match post_file.write_all(content.as_bytes()) {
        Err(e) => return format!("Error: {}", e).to_owned(),
        _ => {}
    };
    return "File uploaded succesfully!".to_owned();
}

#[tauri::command]
unsafe fn login(url: &str, pass: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(pass);
    let result = hex::encode(hasher.finalize());
    let content = Password {
        hash: result.clone(),
    };
    let client = reqwest::blocking::Client::new();
    let res = match client
        .post(format!("{}/api/checkpass_16289461", url))
        .json(&content)
        .send()
    {
        Ok(s) => s,
        _ => return false,
    };
    match res.text().unwrap().as_str() {
        "1" => {
            unsafe {
                URL = url.to_owned();
                HASH = result;
            }
            return true;
        }
        _ => return false,
    }
}
fn main() {
    unsafe {
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![save_post, login])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
