use crate::{HASH, URL};
use serde::Serialize;



#[derive(Serialize)] 
struct ReqBody {
    hash: String,
    id: i32
}

#[tauri::command]
pub fn delete_post(id: String) -> i32 {
    let reqbody = ReqBody {hash: HASH.lock().unwrap().to_string(), id: id.parse().unwrap()};
    let client = reqwest::blocking::Client::new();
    let status = match client
        .post(format!("{}/api/delete_post", URL.lock().unwrap()))
        .json(&reqbody)
        .send()
    {
        Ok(_) => 0,
        Err(_) => 1, 
    };
    return status;
}
