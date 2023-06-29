use serde::Serialize;
use crate::{URL, HASH};
use sha2::{Digest, Sha256};
#[derive(Serialize)]
struct Password {
    hash: String,
}

#[tauri::command]
pub fn login(url: &str, pass: &str) -> bool {
    let mut mutex_url = URL.lock().unwrap();
    let mut mutex_hash = HASH.lock().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(pass);
    let result = hex::encode(hasher.finalize());
    let content = Password {
        hash: result.clone(),
    };
    let client = reqwest::blocking::Client::new();
    let res = match client
        .post(format!("{}/api/checkpass", url))
        .json(&content)
        .send()
    {
        Ok(s) => s,
        _ => return false,
    };
    match res.text().unwrap().as_str() {
        "1" => {
                mutex_url.push_str(url);
                mutex_hash.push_str(result.as_str());
                return true
            }
        
        _ => return false,
    }
}