use std::fmt::format;
use std::{fs, io};
use std::collections::HashMap;
use std::fs::{File, create_dir, create_dir_all};
use std::io::{Read, Seek, Write};
use std::path::Path;
use std::process::exit;
use futures::future::{err, SelectAll};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use inquire::Confirm;
use downloader::{Download,downloader::Builder};
use std::time::Duration;
use zip::ZipArchive;
use std::env;
use std::io::SeekFrom::Current;
use downloader::Verification::Failed;
use futures::Stream;

#[derive(Serialize, Deserialize, Debug)]
struct TAG_ASSETS {
    // url: String,
    id: i32,
    name: String,
    content_type: String,
    state: String,
    size: i32,
    browser_download_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct TAG_INFO {
    // url: String,
    html_url: String,
    id: i32,
    tag_name: String,
    tarball_url: String,
    body: String,
    published_at: String,
    assets: Vec<TAG_ASSETS>
}

#[derive(Serialize, Deserialize, Debug, Default)]
enum APP_TYPE {
    #[default]
    Unknown,
    HitboxOverlay,
    WakeupTool
}

impl APP_TYPE {
    fn name(self) -> String {
        match self {
            APP_TYPE::HitboxOverlay => {"hitbox_overlay".to_string()}
            APP_TYPE::WakeupTool => {"wakeup_tool".to_string()}
            _ => {"".to_string()}
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AppStruct {
    repo_owner: String,
    repo_name: String,
    // App type identifier
    #[serde(default)]
    app_type: APP_TYPE,
    // To update with each version
    #[serde(default)]
    id: i32,
    #[serde(default)]
    tag_name: String,
    #[serde(default)]
    published_at: String,
    #[serde(default)]
    url_source_version: String
}

impl AppStruct {
    fn get_app_name(&self) -> String {
        format!("{}/{}",self.repo_owner,self.repo_name).to_string()
    }

    fn download_mod(&self,destination_dir: &String, tag_info: &TAG_INFO) {
        let mut assets_whitelist:Vec<String> = vec![];

        match self.app_type {
            APP_TYPE::Unknown => {}
            APP_TYPE::WakeupTool => {
                assets_whitelist = vec![
                    format!("GGXrdReversalTool.{}.zip",tag_info.tag_name), // Iquis
                    format!("GGXrdReversalTool-{}.zip",tag_info.tag_name) // kkots
                ];
            }
            APP_TYPE::HitboxOverlay => {
                assets_whitelist = vec!["ggxrd_hitbox_overlay.zip".to_string()];
            }
        }

        let mut matched_assets_list: Vec<&TAG_ASSETS> = vec![];

        for asset in &tag_info.assets {
            if assets_whitelist.contains(&asset.name) {
                matched_assets_list.push(asset);
            }
        }

        for matched_asset in &matched_assets_list {
            download_file_to_path(matched_asset.browser_download_url.to_string(), destination_dir.to_string())
        }

        for matched_asset in matched_assets_list {
            if matched_asset.name.ends_with(".zip") {
                unzip_file(format!("{}/{}",destination_dir.to_string(),matched_asset.name),destination_dir.to_string());
            }
        }
    }

}

impl AppStruct {
    #[tokio::main]
    async fn get_latest_tag(&self) -> Result<TAG_INFO, reqwest::Error> {
        // ➜  ~ curl -L \
        // -H "Accept: application/vnd.github+json" \
        // -H "X-GitHub-Api-Version: 2022-11-28" \
        // https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211/releases/latest

        let repo_url_latest: String = format!("{}/releases/latest",self.get_api_repo_url());

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("Accept","application/vnd.github+json".parse().unwrap());
        headers.insert("GitHub-Api-Version","2022-11-28".parse().unwrap());

        // releases/latest
        let client = reqwest::Client::builder().user_agent("Script-Check-Xrd-Tools").build();
        let response = client.unwrap().get(&repo_url_latest).headers(headers).send().await?;
        let response_status = response.status();
        let mut tag_info: TAG_INFO = TAG_INFO {
            // url: "".to_string(),
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
                println!("Unknown error. Status code {} when getting the latest tag for the repository {}",other, self.get_api_repo_url());
            }
        }

        Ok(tag_info)
    }

    fn get_repo_url(&self) -> String{
        format!("https://github.com/{}/{}",self.repo_owner,self.repo_name).to_string()
    }
    fn get_api_repo_url(&self) -> String{
        format!("https://api.github.com/repos/{}/{}",self.repo_owner,self.repo_name).to_string()
    }
}

// struct APP_VECTOR {
//     vec: Vec<APP>
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct APP_DB {
//     apps: HashMap<String,AppStruct>,
//     #[serde(default)]
//     db_location: String
// }
//
// impl APP_DB {
//     fn init_default_apps_config(&mut self) {
//         let mut new_app_hashmap: HashMap<String,AppStruct> = HashMap::new();
//         let default_repos_list = vec![
//             "https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211".to_string(),
//         ];
//
//         for repository_url in default_repos_list {
//             let app = AppStruct {
//                 repo_owner: "".to_string(),
//                 repo_name: "".to_string(),
//                 url: repository_url.to_string(),
//                 id: 0,
//                 tag_name: "".to_string(),
//                 published_at: "".to_string(),
//                 app_type: Default::default(),
//                 url_source_version: "".to_string(),
//             };
//             new_app_hashmap.insert(app.get_app_name(),app);
//         }
//         self.apps = new_app_hashmap;
//     }
//
//     fn create_new_db(&mut self, file_path: String) -> std::io::Result<()> {
//         // println!("Creating db in '{}'", file_path);
//         // // let file = File::create(file_path)?; //create empty file
//         // // drop(file);
//         // &self.init_default_apps_config();
//         // println!("{:#?}",self);
//         // self.save_db_config()?;
//         // // let config_string = serde_json::to_string(&self.apps)?;
//         //
//         // // file.write_all(config_string.as_bytes())?;
//         Ok(())
//     }
//
//     fn save_db_config(&mut self) -> std::io::Result<()>  {
//         // // &self.recreate_config();
//         // // let config_string = serde_json::to_string(&self.apps)?;
//         //
//         // let mut file = File::create(&self.db_location)?;
//         //
//         // &self.init_default_apps_config();
//         // let config_string = serde_json::to_string(&self.apps)?;
//         // println!("{:#?}",config_string);
//         // println!("!!");
//         // file.write_all(config_string.as_bytes())?;
//         Ok(())
//
//     }
//
//     fn replace_old_tag(mut self, old_app: AppStruct, tag_info: TAG_INFO){
//         // self = tag_info;
//
//     }
// }

//
// struct OLD_CONFIG {
//     mods_folder_path: String,
//     _db_file_name: String,
//     app_db: APP_DB
// }
//
// impl OLD_CONFIG {
//     fn get_db_path(&self) -> String {
//         format!("{}/{}", self.mods_folder_path, self._db_file_name)
//     }
//
//     fn check_db_exists(&mut self, create_db: bool) -> bool {
//         let mut is_present:bool=false;
//         is_present = Path::new(&self.get_db_path()).exists();
//
//         match (is_present,create_db) {
//             (true,false) | (true, true) => {
//                 println!("DB found");
//             }
//             (false,true) => {
//                 println!("DB not found");
//                 if let Err(e) = self.app_db.create_new_db(self.get_db_path()) {
//                     println!("Error: {}", e);
//                     println!("Error creating file.\nExiting...");
//                     exit(1);
//                 }
//                 println!("New DB created");
//             }
//             _ => {}
//         }
//
//         is_present
//     }
//     fn init(&mut self){
//         self.app_db.db_location=self.get_db_path()
//     }
// }


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(default)]
    apps: HashMap<String,AppStruct>
}

impl Config {
    fn set_default_apps (&mut self) {
        let mut new_app_hashmap: HashMap<String,AppStruct> = HashMap::new();
        let mut holder_apps_vector: Vec<AppStruct> = vec![];

        // Hitbox Overlay
        holder_apps_vector.push(
            AppStruct{
                repo_owner: "kkots".to_string(),
                repo_name: "ggxrd_hitbox_overlay_2211".to_string(),
                id: 0,
                tag_name: "".to_string(),
                published_at: "".to_string(),
                app_type: APP_TYPE::HitboxOverlay,
                url_source_version: "".to_string(),
            }
        );

        // Wake up tool Iquis
        holder_apps_vector.push(
            AppStruct{
                repo_owner: "Iquis".to_string(),
                repo_name: "rev2-wakeup-tool".to_string(),
                id: 0,
                tag_name: "".to_string(),
                published_at: "".to_string(),
                app_type: APP_TYPE::WakeupTool,
                url_source_version: "".to_string(),
            }
        );

        // Wake up tool kkots
        holder_apps_vector.push(
            AppStruct{
                repo_owner: "kkots".to_string(),
                repo_name: "rev2-wakeup-tool".to_string(),
                id: 0,
                tag_name: "".to_string(),
                published_at: "".to_string(),
                app_type: APP_TYPE::WakeupTool,
                url_source_version: "".to_string(),
            }
        );

        for app in holder_apps_vector {
            new_app_hashmap.insert(app.get_app_name(),app);
        }

        self.apps = new_app_hashmap;
    }

    fn get_db_dir_path(&mut self) -> String {
        match env::var("XRD_MOD_FOLDER") {
            Ok(env_val) => {
                println!("XRD_MOD_FOLDER env is set to: {}. Overwriting executable location.",env_val);
                env_val
            }
            _ => {
                match env::current_exe() {
                    Ok(exe_path) => {
                        // println!("Path of this executable is: {}",exe_path.display());
                        exe_path.parent().unwrap().to_str().unwrap().to_string()
                    }
                    Err(e) => {
                        println!("failed to get current exe path: {e}");
                        exit(1);
                    }
                }
            }
        }


    }
    fn get_db_path(&mut self) -> String {
        format!("{}/{}", self.get_db_dir_path(), "db.json")
    }
    // fn get_apps_hashmap(&self) -> HashMap<String,&AppStruct>{
    //     let mut apps_hashmap: HashMap<String,&AppStruct> = HashMap::new();
    //
    //     for app in &self.apps {
    //         apps_hashmap.insert(app.get_app_name(),app);
    //     }
    //
    //     apps_hashmap
    // }
    // fn get_app_from_appname(&self, app_name: String) -> Result<&AppStruct, bool> {
    //     for app in &self.apps{
    //         if app.get_app_name() == app_name {
    //             return Ok(app);
    //         }
    //         else { continue }
    //     }
    //     Err(true)
    // }
}

fn print_different_versions(current:&AppStruct,latest:&TAG_INFO) {
    println!("Checking updates for app: {}",current.get_app_name());

    if current.tag_name == latest.tag_name {
        println!("[✅ ] APP {} is up to date!",current.get_app_name());
    } else {
        println!("[⚠️] APP {} has a new version detected.",current.get_app_name());

        // Version
        println!("Version:\t'{}' -> '{}'",current.tag_name,latest.tag_name);
        // Published date
        println!("Published date: '{}' -> '{}'",current.published_at,latest.published_at);
        // Source URL
        println!("Source URL: '{}'",latest.html_url);
        // Print notes
        println!("Version notes:\n============\n{}\n============",latest.body.replace("\\n","\n").replace("\\r",""));
    }
}



struct Manager {
    config: Config
}
impl Manager {
    fn load_config(&self){
        // TODO

    }
    fn save_config(&mut self) -> std::io::Result<()>  {
        // &self.recreate_config();
        // let config_string = serde_json::to_string(&self.apps)?;

        let mut file = File::create(self.config.get_db_path())?;

        let config_string = serde_json::to_string(&self.config)?;
        file.write_all(config_string.as_bytes())?;
        Ok(())
    }

    fn get_latest_tags_hash_map(&self) -> HashMap<String, TAG_INFO> {

        let mut tags_hashmap:HashMap<String, TAG_INFO> =HashMap::new();
        for (appname,appstruct) in &self.config.apps {
            let result = appstruct.get_latest_tag();
            match result {
                Ok(new_tag) => {
                    tags_hashmap.insert(appstruct.get_app_name(), new_tag);
                }
                Err(e) => {
                    println!("Error getting tag for app '{}': << {} >>", appstruct.get_app_name(), e);
                    exit(1);
                }
            }
        }
        tags_hashmap

    }

    fn update_app(&mut self, app_name: &String, latest_tag_info: &TAG_INFO) {
        // Create mod folder if required.
        let modpath_dir = &format!("{}/{}", self.config.get_db_dir_path(), app_name);

        let mut is_dir:bool=Path::new(modpath_dir).is_dir();

        match is_dir {
            true => {}
            false => {
                if let Err(e) = create_dir_all(modpath_dir) {
                    println!("Error: {}", e);
                    println!("Error creating dir.\nExiting...");
                    exit(1);
                }
                println!("Created directory for the mod {} located at '{}'", app_name, modpath_dir)
            }
        }


        // Respective update
        match self.config.apps.get(app_name) {
            Some(current_app) => {
                if current_app.tag_name == latest_tag_info.tag_name.to_string() {
                    println!("[✅ ] APP {} is up to date, skipping...",current_app.get_app_name());
                } else {
                    println!("[⚠️] Updating '{}'",current_app.get_app_name());
                    match current_app.app_type {
                        APP_TYPE::Unknown => {println!("[🚫] App '{}' doesn't have a update procedure. Skipping", current_app.get_app_name())}
                        APP_TYPE::WakeupTool => {
                            current_app.download_mod(modpath_dir, latest_tag_info);
                        }
                        APP_TYPE::HitboxOverlay => {
                            current_app.download_mod(modpath_dir, latest_tag_info);
                        }
                    }
                }
            }
            None => {
                println!("App '{}' not found. Skipping for tag with url '{}'", app_name, latest_tag_info.html_url);
            }
        }


        // Update app ionfo
        let mut app_to_update = self.config.apps.get_mut(app_name).unwrap();
        app_to_update.tag_name = latest_tag_info.tag_name.to_string();
        app_to_update.published_at = latest_tag_info.published_at.to_string();
        app_to_update.url_source_version = latest_tag_info.html_url.to_string();
        app_to_update.id = latest_tag_info.id;

    }




    fn update_all(&mut self){
        let tags_hashmap: HashMap<String, TAG_INFO> = self.get_latest_tags_hash_map();

        for (app_name,latest_tag_info) in &tags_hashmap {
            match self.config.apps.get(app_name) {
                Some(current_app)  => {
                    print_different_versions(current_app,latest_tag_info);
                }
                None => {
                    println!("App '{}' not found. Skipping for tag with url '{}'",app_name,latest_tag_info.html_url);
                }
            }
        }

        let ans = Confirm::new("Do you wish to update to the latest version?").
            with_default(false).
            with_help_message("This will update all the mentioned apps").
            prompt();

        match ans {
            Ok(true) => {
                for (app_name,latest_tag_info) in &tags_hashmap {
                    self.update_app(app_name, latest_tag_info);
                }
            },
            Ok(false) => println!("That's too bad, I've heard great things about it."),
            Err(_) => println!("Error with the input."),
        }

        match self.save_config(){
            Ok(_) => {
                println!("Successfully saved the configuration.")
            }
            Err(e) => {
                println!("Error Saving the configuration: '{}'",e)
            }
        }
    }
}

fn main() {
    let mut manager = Manager {
        config: Config{ apps: HashMap::new() }
    };

    manager.load_config();

    manager.config.set_default_apps();

    // println!("{:#?}",manager.config);
    // println!("{:#?}",manager.config.get_db_location());

    // for app in &manager.config.apps {
    //     println!("{:?}",app);
    // }

    manager.update_all();

    // println!("EOF apps print"); // TODO REMOVE VISUAL PRINT
    // for app in &manager.config.apps {
    //     println!("{:?}",app);
    // }
}


//
// fn check_app_updates(config: OLD_CONFIG){
//     for mut app in config.app_db.apps {
//         println!("Checking updates for app {}",app.url);
//         let result = app.get_latest_tag();
//
//         if let Err(e) = result {
//             println!("Error: {}", e);
//             exit(1);
//         }
//         let latest_tag: TAG_INFO = result.unwrap();
//         // println!("{:#?}", latest_tag);
//
//         if app.tag_name == latest_tag.tag_name && app.published_at == latest_tag.published_at{
//             println!(" [✅ ] Latest tag already in use.");
//         } else {
//             println!(" [⚠️] Differences have been found!");
//
//             println!("\tCurrent tag:");
//             println!("\t  Name: '{}'",app.tag_name);
//             println!("\t  Published date: '{}'",app.published_at);
//
//             println!("\tLatest tag:");
//             println!("\t  Name: '{}'",latest_tag.tag_name);
//             println!("\t  Published date: '{}'",latest_tag.published_at);
//             let ans = Confirm::new("Do you wish to update to the latest version?")
//                 .with_default(false)
//                 .prompt();
//                 // .with_help_message("This data is stored for good reasons")
//
//             match ans {
//                 Ok(true) => {
//                     app.update_app(&format!("{}/{}",config.mods_folder_path,app.get_app_name()),latest_tag);
//                     // config.app_db.save_db_config()
//                 },
//                 Ok(false) => println!("That's too bad, I've heard great things about it."),
//                 Err(_) => println!("Error with the input."),
//             }
//         }
//     }
// }

// fn load_apps(config: &mut OLD_CONFIG) -> std::io::Result<()> {
//     // let mut file = File::open(config.get_db_path())?;
//     // let mut contents = String::new();
//     // file.read_to_string(&mut contents)?;
//     //
//     // config.app_db.apps = serde_json::from_str(&contents).unwrap();
//
//     Ok(())
// }

// fn load_db(config: &CONFIG){
//     check_db_exists(config.db_path(),true);
//     return;
// }

fn download_file_to_path(file_url: String, destination_dir: String){
    // Download overlay.zip
    let file_to_download = Download::new(&file_url);
    let destination_file_path = &format!("{}/{}", destination_dir, file_to_download.file_name.to_str().unwrap().to_string());

    // Check if file already exists
    let mut is_present:bool=Path::new(destination_file_path).exists();
    let mut is_dir:bool=Path::new(destination_file_path).is_dir();

    match (is_present,is_dir) {
        (true,false) => {
            println!("A file with the name '{}' already exists, proceeding with the deletion.",destination_file_path);
            fs::remove_file(destination_file_path);
        }
        (true,true) => {
            // Error won't delete a folder
            println!("The file '{}' cannot be downloaded due to a directory having the exact same name.",destination_file_path);
            exit(1);
        }
        _ => {}

    }


    // let mut is_dir:bool=Path::new(mod_folder).is_dir();

    // copy pasta
    // https://github.com/hunger/downloader
    let mut dl = Builder::default()
        .connect_timeout(Duration::from_secs(4))
        .download_folder(Path::new(&destination_dir))
        .parallel_requests(8)
        .build()
        .unwrap();

    let response = dl.download(&[file_to_download]).unwrap(); // other error handling

    response.iter().for_each(|v| match v {
        Ok(v) => println!("Downloaded: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    });
}

fn unzip_file(zip_file_path: String, unzip_dir:String){
    let zipfile = std::fs::File::open(zip_file_path).unwrap();

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = format!("{}/{}",unzip_dir,file.name());
        println!("{:?}", outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath);
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath,
                file.size()
            );
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}