use std::{fs::File, path::Path, ffi::OsStr};
use base64::{Engine as _, engine::general_purpose};
use chrono::prelude::*;
use std::io::Read;
use serde::Serialize;

use crate::{HASH, URL};
#[derive(Serialize)]
struct Post {
    hash: String,
    title: String,
    image: String,
    date: String,
    content: String,
    image_ext: String
}

#[tauri::command]
pub fn save_post(title: &str, img_path: &str, content: &str) -> String {
    println!("1");
    let mut image = match File::open(img_path) {
        Ok(x) => x,
        Err(why) => {return format!("Cannot open the file: {}", why);}
    }; 
    let mut buf = Vec::new();
    match image.read_to_end(&mut buf) {
        Ok(_) => {},
        Err(why) => {return format!("Cannot read contents of the file: {}", why);}
    };
    let b64image = general_purpose::URL_SAFE_NO_PAD.encode(buf);
    let image_ext = match Path::new(img_path).extension().and_then(OsStr::to_str){
        Some(x) => x,
        _ => {return "Cannot get the file extension".to_string()}
    }; 
    let date = Local::now().format("%B %e, %Y").to_string();
    let client = reqwest::blocking::Client::new();
    let req_content = Post {
        hash: HASH.lock().unwrap().to_string(),
        title: title.to_string(), 
        date: date,
        image: b64image,
        content: content.to_string(),
        image_ext: image_ext.to_string(),
    };
    let res = match client
        .post(format!("{}/api/new_post", URL.lock().unwrap()))
        .json(&req_content)
        .send()
    {
        Ok(x) => x,
        Err(why) => {return format!("Error: {}", why);}
    };
    println!("{:?}", res);
    return "File uploaded succesfully!".to_owned();
}