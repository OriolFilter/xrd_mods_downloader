use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use futures::future::err;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use inquire::Confirm;

#[derive(Serialize, Deserialize, Debug)]
struct TAG_ASSETS {
    url: String,
    id: i32,
    name: String,
    content_type: String,
    state: String,
    size: i32,
    browser_download_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct TAG_INFO {
    url: String,
    html_url: String,
    id: i32,
    tag_name: String,
    tarball_url: String,
    body: String,
    published_at: String,
    assets: Vec<TAG_ASSETS>
}


#[derive(Serialize, Deserialize, Debug)]
struct APP {
    url: String,
    #[serde(default)]
    id: i32,
    #[serde(default)]
    tag_name: String,
    #[serde(default)]
    published_at: String,
}

impl APP {
    #[tokio::main]
    async fn get_latest_release(&self) -> Result<TAG_INFO, reqwest::Error> {
        // ➜  ~ curl -L \
        // -H "Accept: application/vnd.github+json" \
        // -H "X-GitHub-Api-Version: 2022-11-28" \
        // https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211/releases/latest

        let repo_url_latest: String = format!("{}/releases/latest",self.url);

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("Accept","application/vnd.github+json".parse().unwrap());
        headers.insert("GitHub-Api-Version","2022-11-28".parse().unwrap());

        // releases/latest
        let client = reqwest::Client::builder().user_agent("Script-Check-Xrd-Tools").build();
        let response = client.unwrap().get(&repo_url_latest).headers(headers).send().await?;
        let response_status = response.status();
        let mut tag_info: TAG_INFO = TAG_INFO {
            url: "".to_string(),
            html_url: "".to_string(),
            id: 0,
            tag_name: "".to_string(),
            tarball_url: "".to_string(),
            body: "".to_string(),
            published_at: "".to_string(),
            assets: vec![],
        };

        match response_status {
            reqwest::StatusCode::OK => {
                // println!("{:#?}",&response.json().await.unwrap());
                tag_info = response.json().await.unwrap();
            }
            other => {
                println!("Unknown error. Status code {} when getting the latest tag for the repository {}",other, self.url);
            }
        }

        Ok(tag_info)
    }
    fn update_app(&self,destination_path: &String ,tag_info: TAG_INFO) {
        // https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211/releases/latest
        println!("Updating app {} to tag {}",self.url,tag_info.id);

        match self.url.as_str() {
            "https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211" => {
                download_hitbox_overlay(destination_path, tag_info);
            }
            _ => {}
        }

    }
}

// struct APP_VECTOR {
//     vec: Vec<APP>
// }

#[derive(Serialize, Deserialize, Debug)]
struct APP_DB {
    apps: Vec<APP>
}

impl APP_DB {
    fn recreate_config(&mut self) {
        let mut new_app_vector: Vec<APP> = vec![];
        let default_repos_list = vec![
            "https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211".to_string(),
        ];

        for repository_url in default_repos_list {
            new_app_vector.push(APP{
                url: repository_url.to_string(),
                id: 0,
                tag_name: "".to_string(),
                published_at: "".to_string(),
            })
        }
        self.apps = new_app_vector;
    }

    fn create_new_db(&mut self, file_path: String) -> std::io::Result<()> {
        println!("Creating db in '{}'", file_path);
        let mut file = File::create(file_path)?;

        &self.recreate_config();
        let config_string = serde_json::to_string(&self.apps)?;

        file.write_all(config_string.as_bytes())?;
        Ok(())
    }

    fn save_db_config(&mut self, file_path: String) -> std::io::Result<()>  {
        &self.recreate_config();
        // let config_string = serde_json::to_string(&self.apps)?;

        let mut file = File::open(file_path)?;

        &self.recreate_config();
        let config_string = serde_json::to_string(&self.apps)?;

        file.write_all(config_string.as_bytes())?;
        Ok(())

    }
}

struct CONFIG{
    mods_folder_path: String,
    _db_file_name: String,
    app_db: APP_DB
}

impl CONFIG {
    fn db_path(&self) -> String {
        format!("{}/{}", self.mods_folder_path, self._db_file_name)
    }

    fn check_db_exists(&mut self, create_db: bool) -> bool {
        let mut is_present:bool=false;
        is_present = Path::new(&self.db_path()).exists();

        match (is_present,create_db) {
            (true,false) | (true, true) => {
                println!("DB found");
            }
            (false,true) => {
                println!("DB not found");
                if let Err(e) = self.app_db.create_new_db(self.db_path()) {
                    println!("Error: {}", e);
                    println!("Error creating file.\nExiting...");
                    exit(1);
                }
                println!("New DB created");
            }
            _ => {}
        }

        is_present
    }
}

fn main() {
    let mods_folder_path: String="/tmp/xrd_mods".to_string();
    let mut config = CONFIG{
        mods_folder_path: mods_folder_path,
        _db_file_name: "db.json".to_string(),
        app_db: APP_DB { apps: vec![] },
    };
    // load_db(&config);
    config.check_db_exists(true);

    if let Err(e) = load_apps(&mut config) {
        println!("Error loading apps from the DB");
        exit(1);
    }
    println!("Apps load from the DB");

    // for app in &config.app_db.apps{
    //     println!("{:?}",app);
    // }

    // let xrd_folder: String="/home/goblin/.local/share/Steam/steamapps/common/GUILTY GEAR Xrd -REVELATOR-/".to_string();

    // https://github.com/kkots/ggxrd_hitbox_overlay_2211

    // https://github.com/Iquis/rev2-wakeup-tool/

    // download_function_tool()
    // download_hitbox_overlay();
    check_app_updates(config);

}



fn check_app_updates(config: CONFIG){
    for app in config.app_db.apps {
        println!("Checking updates for app {}",app.url);
        let result = app.get_latest_release();

        if let Err(e) = result {
            println!("Error: {}", e);
            exit(1);
        }
        let latest_tag: TAG_INFO = result.unwrap();
        // println!("{:#?}", latest_tag);

        if app.tag_name == latest_tag.tag_name && app.published_at == latest_tag.published_at{
            println!(" [✅] Latest tag already in use.");
        } else {
            println!(" [⚠️] Differences have been found!");

            println!("\tCurrent tag:");
            println!("\t  Name: '{}'",app.tag_name);
            println!("\t  Published date: '{}'",app.published_at);

            println!("\tLatest tag:");
            println!("\t  Name: '{}'",latest_tag.tag_name);
            println!("\t  Published date: '{}'",latest_tag.published_at);
            let ans = Confirm::new("Do you wish to update to the latest version?")
                .with_default(false)
                .prompt();
                // .with_help_message("This data is stored for good reasons")

            match ans {
                Ok(true) => {
                    app.update_app(&config.mods_folder_path,latest_tag);
                },
                Ok(false) => println!("That's too bad, I've heard great things about it."),
                Err(_) => println!("Error with the input."),
            }
        }
    }
}

fn load_apps(config: &mut CONFIG) -> std::io::Result<()> {
    let mut file = File::open(config.db_path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut app_list = vec![APP{
        url: "".to_string(),
        id: 0,
        tag_name: "".to_string(),
        published_at: "".to_string(),
    }];

    config.app_db.apps = serde_json::from_str(&contents).unwrap();

    Ok(())
}

// fn load_db(config: &CONFIG){
//     check_db_exists(config.db_path(),true);
//     return;
// }

fn download_hitbox_overlay(destination_path: &String, tag_info: TAG_INFO) {
    println!("{:#?}",tag_info);
    // Download tarball




}


// fn download_app(config: CONFIG, url: String,tag_info: TAG_INFO){
//     match url.as_str() {
//         "https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211" => {
//              download_hitbox_overlay(config.mods_folder_path);
//         }
//         other => {println!("No download option found that matches the URL  '{other}', cannot proceed with the download.")}
//     }
//
// }




fn check_versions_differ(repo_url: String, tag_id: i32, destination_path: String) {
    // if file doesn't exist, create file.
    // check_db_exists(destination_path)

}