use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize)]
struct TAG_INFO {
    url: String,
    html_url: String,
    id: i32,
    tag_name: String,
    tarball_url: String,
    body: String,
    published_at: String
}

struct APP {
    url: String,
    id: String,
    tag_name: String,
    published_at: String
}

// struct APP_VECTOR {
//     vec: Vec<APP>
// }

struct CONFIG{
    storage_path: String,
    _db_file_name: String,
    apps: Vec<APP>
}

impl CONFIG {
    fn db_path(&self) -> String {
        format!("{}/{}",self.storage_path,self._db_file_name)
    }
}

fn main() {
    let destination_path: String="/tmp/xrd_mods".to_string();
    let config = CONFIG{
        storage_path: destination_path,
        _db_file_name: "db.json".to_string(),
        apps: vec![]
    };
    // load_db(&config);
    check_db_exists(config.db_path(),true);

    load_apps(&config);

    // let xrd_folder: String="/home/goblin/.local/share/Steam/steamapps/common/GUILTY GEAR Xrd -REVELATOR-/".to_string();

    // https://github.com/kkots/ggxrd_hitbox_overlay_2211

    // https://github.com/Iquis/rev2-wakeup-tool/

    // download_function_tool()
    download_hitbox_overlay(config.storage_path);

}

fn load_apps(config: &CONFIG) -> std::io::Result<()> {
    let mut file = File::open(config.db_path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // println!("{:?}", contents);

    let value: Value = serde_json::from_str(&*contents)?;
    println!("{:?}",value);

    Ok(())
}

// fn load_db(config: &CONFIG){
//     check_db_exists(config.db_path(),true);
//     return;
// }

#[tokio::main]
async fn get_latest_release(repo_url: String) -> Result<String, reqwest::Error> {
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
    let response = client.unwrap().get(&repo_url_latest).headers(headers).send().await?;
    let response_status = response.status();
    let mut response_json: TAG_INFO = TAG_INFO {
        url: "".to_string(),
        html_url: "".to_string(),
        id: 0,
        tag_name: "".to_string(),
        tarball_url: "".to_string(),
        body: "".to_string(),
        published_at: "".to_string(),
    };

    match response_status {
        reqwest::StatusCode::OK => {
            println!("Success");
            response_json = response.json().await.unwrap();
        }
        other => {
            println!("Unknown error. Status code {other}");
        }
    }

    println!("result = {:?}", response_status);
    // println!("result = {:?}", reposnse_text);

    Ok(response_json.url)
}

fn download_hitbox_overlay(_destination_path: String) {
    // let repo_url: String = "https://github.com/kkots/ggxrd_hitbox_overlay_2211".to_string();
    let repo_url: String = "https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211".to_string();

    println!("download_hitbox_overlay()");
    let result = get_latest_release(repo_url);
    if let Err(e) = result {
        println!("Error: {}", e);
        exit(1);
    }
    println!("Latest release: {:?}", result.unwrap());
}


// fn download_function_tool() {
//
// }

fn create_new_db(file_path: String) -> std::io::Result<()> {
    println!("Creating db in '{}'", file_path);
    let mut file = File::create(file_path)?;
    file.write_all(b"[]")?;
    Ok(())
}

fn check_db_exists(db_path: String, create_db: bool) -> bool {
    let mut is_present:bool=false;
    is_present = Path::new(&db_path).exists();

    match (is_present,create_db) {
        (true,false) | (true, true) => {
            println!("DB found");
        }
        (false,true) => {
            println!("DB not found");
            if let Err(e) = create_new_db(db_path) {
                println!("Error: {}", e);
                println!("Error creating file.\nExiting...");
                exit(1);
            }
            println!("New DB created");
        }
        _ => {}
    }

    return is_present;
}

fn check_versions_differ(repo_url: String, tag_id: i32, destination_path: String) {
    // if file doesn't exist, create file.
    // check_db_exists(destination_path)

}