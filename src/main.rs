use std::collections::HashMap;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;



#[derive(Serialize, Deserialize)]
struct TAG_INFO {
    url: String,
    html_url: String,
    id: i32,
    tag_name: String,
    tarball_url: String,
    body: String,
}

fn main() {
    let destination_path: String="/tmp/xrd_mods/".to_string();
    // let xrd_folder: String="/home/goblin/.local/share/Steam/steamapps/common/GUILTY GEAR Xrd -REVELATOR-/".to_string();

    // https://github.com/kkots/ggxrd_hitbox_overlay_2211

    // https://github.com/Iquis/rev2-wakeup-tool/

    // download_function_tool()
    download_hitbox_overlay(destination_path);


}


#[tokio::main]
async fn get_latest_release(repo_url: String) -> String {
    // ➜  ~ curl -L \
    // -H "Accept: application/vnd.github+json" \
    // -H "X-GitHub-Api-Version: 2022-11-28" \
    // https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211/releases/latest

    let repo_url_latest: String = format!("{repo_url}/releases/latest");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept","application/vnd.github+json".parse().unwrap());
    headers.insert("GitHub-Api-Version","2022-11-28".parse().unwrap());

    // releases/latest
    let client = reqwest::Client::builder().user_agent("Script-Check-Xrd-Tools").build();
    let response = client.unwrap().get(&repo_url_latest).headers(headers).send().await;
    let response_json: TAG_INFO = response.unwrap().json().await.unwrap();

    println!("{:?}", response_json.url);

    return repo_url_latest;
}

fn download_hitbox_overlay(_destination_path: String) {
    // let repo_url: String = "https://github.com/kkots/ggxrd_hitbox_overlay_2211".to_string();
    let repo_url: String = "https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211".to_string();

    println!("Latest release: {}", get_latest_release(repo_url));
}


// fn download_function_tool() {
//
// }