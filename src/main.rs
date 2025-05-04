use std::fmt::format;
use std::{fs, io};
use std::collections::HashMap;
use std::fs::{File, create_dir};
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
    #[serde(default)]
    url: String, // TODO not use
    #[serde(default)]
    id: i32,
    #[serde(default)]
    tag_name: String,
    #[serde(default)]
    published_at: String,
    #[serde(default)]
    app_type: APP_TYPE,
    #[serde(default)]
    url_source_version: String
}

impl AppStruct {
    fn get_app_name(&self) -> String {
        format!("{}/{}",self.repo_owner,self.repo_name).to_string()
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
                println!("Unknown error. Status code {} when getting the latest tag for the repository {}",other, self.url);
            }
        }

        Ok(tag_info)
    }
    // fn update_app(&mut self, mod_folder: &String, tag_info: TAG_INFO) {
    //     // https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211/releases/latest
    //     println!("Updating app {} to tag {}",self.url,tag_info.id);
    //
    //     // check app directory exists
    //     // let mut is_present:bool=Path::new(mod_folder).exists();
    //     let mut is_dir:bool=Path::new(mod_folder).is_dir();
    //
    //     match is_dir {
    //         true => {}
    //         false => {
    //             if let Err(e) = create_dir(mod_folder) {
    //                 println!("Error: {}", e);
    //                 println!("Error creating file.\nExiting...");
    //                 exit(1);
    //             }
    //             println!("Created directory for the mod {} located at '{}'",self.repo_name,mod_folder)
    //         }
    //     }
    //
    //
    //
    //
    //     // Update old values
    //     self.published_at = tag_info.published_at.to_string();
    //     self.id = tag_info.id;
    //
    //     // update app
    //     match self.repo_name.as_str() {
    //         "ggxrd_hitbox_overlay" => {
    //             download_hitbox_overlay(mod_folder, tag_info);
    //         }
    //         _ => {}
    //     }
    // }

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
    apps: HashMap<String,AppStruct>,
    #[serde(default)]
    db_location: String
}

impl Config {
    fn set_default_apps (&mut self) {
        let mut new_app_hashmap: HashMap<String,AppStruct> = HashMap::new();
        let mut holder_apps_vector: Vec<AppStruct> = vec![];

        holder_apps_vector.push(
            AppStruct{
                repo_owner: "kkots".to_string(),
                repo_name: "ggxrd_hitbox_overlay_2211".to_string(),
                url: "".to_string(),
                id: 0,
                tag_name: "".to_string(),
                published_at: "".to_string(),
                app_type: APP_TYPE::HitboxOverlay,
                url_source_version: "".to_string(),
            }
        );

        for app in holder_apps_vector {
            new_app_hashmap.insert(app.get_app_name(),app);
        }

        self.apps = new_app_hashmap;
    }

    fn get_db_location (&mut self) -> String {
        if &self.db_location == &"".to_string() {
            match env::current_exe() {
                Ok(exe_path) => {
                    // println!("Path of this executable is: {}",exe_path.display());
                    self.db_location = exe_path.parent().unwrap().to_str().unwrap().to_string();
                }
                Err(e) => {
                    println!("failed to get current exe path: {e}");
                    exit(1);
                }
            };

        };

        self.db_location.to_string()
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
        println!("[✅] APP {} is up to date!",current.get_app_name());
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
    fn save_config(&self){
        // TODO
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

    fn update_app(&mut self, current:&AppStruct, latest:&TAG_INFO) {
        let updated: bool = false;
        // if current.tag_name == latest.tag_name {
        //     println!("[✅] APP {} is up to date, skipping...",current.get_app_name());
        // } else {
        //     println!("[⚠️] Updating '{}'",current.get_app_name());
        //     match current.app_type {
        //         APP_TYPE::Unknown | APP_TYPE::WakeupTool => {println!("App '{}' doesn't have a update procedure. Skipping", current.get_app_name())}
        //         APP_TYPE::HitboxOverlay => {
        //             download_hitbox_overlay(&self.config.get_db_location(), latest);
        //         }
        //     }
        // }
    }

    fn update_all(&mut self){
        // Get ALL tags -> then compare -> prompt
        let tags_hashmap: HashMap<String, TAG_INFO> = self.get_latest_tags_hash_map();
        // let apps_hashmap: HashMap<String, &AppStruct> = self.config.get_apps_hashmap();

        for (app_name,latest_tag_info) in &tags_hashmap {
            match self.config.apps.get_mut(app_name) {
                Some(current_app)  => {
                    print_different_versions(current_app,latest_tag_info);
                }
                None => {
                    println!("App '{}' not found. Skipping for tag with url '{}'",app_name,latest_tag_info.html_url);
                }
            }
        }

        // let target_app = self.config.get_app_from_appname(self.config.apps[0].get_app_name());
        // println!("{:#?}", target_app);

        // println!("## {:#?}",self.config.apps);
        // println!("## {:#?}",tags_hashmap);

        // println!("Updating all the apps:");
        for (app_name,latest_tag_info) in &tags_hashmap {
            match self.config.apps.get_mut(app_name) {
                Some(current_app) => {
                    println!("{}", self.config.db_location);
                    self.update_app(current_app,latest_tag_info);
                }
                None => {
                    println!("App '{}' not found. Skipping for tag with url '{}'", app_name, latest_tag_info.html_url);
                }
            }
        }
        //     // println!("> {:#?}",app_name);
        //     // println!(">> {:#?}",tag_info);
        //     // let target_app = s elf.config.get_app_from_appname(app_name.to_string());
        //     // for app in &self.config.apps{
        //     //     if app.get_app_name() == app_name.to_string() {
        //     //         self.update_app(app,tag_info);
        //     //     }
        //     //     else { continue }
        //     // }
        //
        //     // match target_app {
        //     //     Ok(appstruct)  => {
        //     //         println!("{:#?}", appstruct);
        //     //         self.update_app(appstruct,tag_info);
        //     //     }
        //     //     Err(false)|Err(true) => {
        //     //         println!("App '{}' not found. Skipping for tag with url '{}'",app_name,tag_info.html_url);
        //     //     }
        //     // }
        // }


        // for mut app in &mut self.config.apps {
        //     app.tag_name="NEW".to_string();
        // }
        // let latest_tags_hashmap: HashMap<String, &TAG_INFO> = self.get_latest_tags_hash_map();

        // for (app_name,TAG_INFO) in latest_tags_hashmap {
        //
        //     // // find the app that matches the tag
        //     // let target_app: AppStruct;
        //     // for app in self.config.apps {
        //     //     match (app.repo_owner,app.repo_name) {
        //     //          // => {}
        //     //         (_, _) => {continue}
        //     //     }
        //     // }
        // }

            // if app.tag_name == latest_tag.tag_name && app.published_at == latest_tag.published_at{
            //     println!(" [✅] Latest tag already in use.");
            // } else {
            //     println!(" [⚠️] Differences have been found!");
            //
            //     println!("\tCurrent tag:");
            //     println!("\t  Name: '{}'",app.tag_name);
            //     println!("\t  Published date: '{}'",app.published_at);
            //
            //     println!("\tLatest tag:");
            //     println!("\t  Name: '{}'",latest_tag.tag_name);
            //     println!("\t  Published date: '{}'",latest_tag.published_at);
            //     let ans = Confirm::new("Do you wish to update to the latest version?")
            //         .with_default(false)
            //         .prompt();
            //     // .with_help_message("This data is stored for good reasons")
            //
            //     match ans {
            //         Ok(true) => {
            //             app.update_app(&format!("{}/{}",config.mods_folder_path,app.app_name),latest_tag);
            //             // config.app_db.save_db_config()
            //         },
            //         Ok(false) => println!("That's too bad, I've heard great things about it."),
            //         Err(_) => println!("Error with the input."),
            //     }
            // }
        // }
    }
}

fn main() {

    let mut manager = Manager {
        config: Config{ apps: HashMap::new(), db_location: "/tmp/xrd_mods".to_string() }
    };

    manager.load_config();

    manager.config.set_default_apps();

    println!("{:#?}",manager.config);
    println!("{:#?}",manager.config.get_db_location());

    for app in &manager.config.apps {
        println!("{:#?}",app);
    }

    manager.update_all();

    println!("EOF apps print"); // TODO REMOVE VISUAL PRINT
    for app in &manager.config.apps {
        println!("{:#?}",app);
    }

    // // let mut config = OLD_CONFIG {
    // //     mods_folder_path: mods_folder_path,
    // //     _db_file_name: "db.json".to_string(),
    // //     app_db: APP_DB { apps: vec![], db_location: "".to_string() },
    // // };
    // config.init();
    // // load_db(&config);
    // config.check_db_exists(true);
    //
    // if let Err(e) = load_apps(&mut config) {
    //     println!("Error loading apps from the DB");
    //     exit(1);
    // }
    // println!("Apps load from the DB");
    //
    // // for app in &config.app_db.apps{
    // //     println!("{:?}",app);
    // // }
    //
    // // let xrd_folder: String="/home/goblin/.local/share/Steam/steamapps/common/GUILTY GEAR Xrd -REVELATOR-/".to_string();
    //
    // // https://github.com/kkots/ggxrd_hitbox_overlay_2211
    //
    // // https://github.com/Iquis/rev2-wakeup-tool/
    //
    // // download_function_tool()
    // // download_hitbox_overlay();
    // check_app_updates(config);

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
//             println!(" [✅] Latest tag already in use.");
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

fn download_hitbox_overlay(destination_path: &String, tag_info: &TAG_INFO) {
    let mut ggxrd_hitbox_overlay_zip: TAG_ASSETS = TAG_ASSETS {
        id: 0,
        name: "".to_string(),
        content_type: "".to_string(),
        state: "".to_string(),
        size: 0,
        browser_download_url: "".to_string(),
    };

    // // Identify assets
    // for asset in tag_info.assets {
    //     match asset.name.as_str() {
    //         "ggxrd_hitbox_overlay.zip" => {ggxrd_hitbox_overlay_zip=asset;}
    //         _ => {}
    //     }
    // }
    //
    // // Download overlay.zip
    // let file_to_download = Download::new(&ggxrd_hitbox_overlay_zip.browser_download_url);
    //
    // // Check if file already exists
    // let mut is_present:bool=Path::new(&format!("{}/{}",destination_path,ggxrd_hitbox_overlay_zip.name)).exists();
    // let mut is_dir:bool=Path::new(&format!("{}/{}",destination_path,ggxrd_hitbox_overlay_zip.name)).is_dir();
    //
    // match (is_present,is_dir) {
    //     (true,false) => {
    //         println!("A file with the name '{}' already exists, proceeding with the deletion.",&format!("{}/{}",destination_path,ggxrd_hitbox_overlay_zip.name));
    //         fs::remove_file(&format!("{}/{}",destination_path,ggxrd_hitbox_overlay_zip.name));
    //     }
    //     (true,true) => {
    //         // Error won't delete a folder
    //         println!("The file '{}' cannot be downloaded due to a directory having the exact same name.",&format!("{}/{}",destination_path,ggxrd_hitbox_overlay_zip.name));
    //         exit(1);
    //     }
    //     _ => {}
    //
    // }
    //
    //
    // // let mut is_dir:bool=Path::new(mod_folder).is_dir();
    //
    // // copy pasta
    // // https://github.com/hunger/downloader
    // let mut dl = Builder::default()
    //     .connect_timeout(Duration::from_secs(4))
    //     .download_folder(Path::new(destination_path))
    //     .parallel_requests(8)
    //     .build()
    //     .unwrap();
    //
    // let response = dl.download(&[file_to_download]).unwrap(); // other error handling
    //
    // response.iter().for_each(|v| match v {
    //     Ok(v) => println!("Downloaded: {:?}", v),
    //     Err(e) => println!("Error: {:?}", e),
    // });
    //
    // install_hitbox_overlay(destination_path.to_string());
}

fn install_hitbox_overlay(download_path: String){
    // copy pasta
    // https://github.com/zip-rs/zip2/blob/master/examples/extract.rs

    let fname = format!("{}/{}",download_path,"ggxrd_hitbox_overlay.zip");
    let zipfile = std::fs::File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = format!("{}/{}",download_path,file.name());
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


// fn download_app(config: CONFIG, url: String,tag_info: TAG_INFO){
//     match url.as_str() {
//         "https://api.github.com/repos/kkots/ggxrd_hitbox_overlay_2211" => {
//              download_hitbox_overlay(config.mods_folder_path);
//         }
//         other => {println!("No download option found that matches the URL  '{other}', cannot proceed with the download.")}
//     }
//
// }