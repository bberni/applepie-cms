use serde::{Serialize, Deserialize};
use crate::{HASH, URL}; 

#[derive(Deserialize)]
struct Post {
    id: i32,
    title: String,
    date: String,
}

#[derive(Serialize)] 
struct ReqBody {
    hash: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    posts: Vec<Post>,
}
#[tauri::command]
pub fn post_list() -> Vec<Vec<String>> {
    let reqbody = ReqBody {hash: HASH.lock().unwrap().to_string()};
    let client = reqwest::blocking::Client::new();
    let res = match client
        .post(format!("{}/api/post_list", URL.lock().unwrap()))
        .json(&reqbody)
        .send()
    {
        Ok(x) => x,
        Err(_) => {return Vec::new()}, 
    };
    println!("{:?}", res);
    let res: ApiResponse = res.json().unwrap();
    let posts = res.posts;
    let mut post_vec: Vec<Vec<String>> = Vec::new(); 
    for post in posts {
        post_vec.push(vec![post.id.to_string(), post.title, post.date]);
    };
    return post_vec;
}