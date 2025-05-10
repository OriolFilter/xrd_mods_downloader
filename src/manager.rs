use crate::stuff::*;

use std::fmt::{format, Write as StdinWrite};
use std::{fs, io};
use std::collections::HashMap;
use std::fs::{File, create_dir, create_dir_all, Permissions};
use std::io::{Error, Read, Seek, Write};
use std::path::Path;
use std::process::{exit, Stdio};
use futures::future::{err, ok, SelectAll};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use inquire::Confirm;
use downloader::{Download,downloader::Builder};
use std::time::Duration;
use zip::ZipArchive;
use std::env;
use std::io::SeekFrom::Current;
use std::ops::BitOr;
use downloader::Verification::Failed;
use futures::Stream;
use std::process::Command;

// Linux imports
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

// Windows imports
#[cfg(target_os = "windows")]
// Get path from Windows registry
use winreg::{RegKey,enums::*};

pub struct Manager {
    pub(crate) config: Config
}

impl Manager {
    pub(crate) fn load_config(&mut self) -> std::io::Result<()> {
        // load config from db.json.
        // otherwise load default config.

        let mut is_present:bool=Path::new(&self.config.get_db_file_path()).exists();

        match is_present {
            false => {
                println!("DB not found. Loading defaults.");
                self.config.set_default_apps();
            }
            _ => {
                let mut file = File::open(self.config.get_db_file_path())?; // Open file
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                self.config = serde_json::from_str(&contents)?;
            }
        }
        Ok(())
    }

    fn save_config(&mut self) -> std::io::Result<()>  {
        let mut file = File::create(self.config.get_db_file_path())?;

        let config_string = serde_json::to_string(&self.config)?;
        file.write_all(config_string.as_bytes())?;
        Ok(())
    }

    fn get_latest_tags_hash_map(&self) -> HashMap<String, TagInfo> {
        let mut tags_hashmap:HashMap<String, TagInfo> =HashMap::new();
        for app_struct in self.config.apps.values() {
            let result = app_struct.get_latest_tag();
            match result {
                Ok(new_tag) => {
                    tags_hashmap.insert(app_struct.get_app_name(), new_tag);
                }
                Err(e) => {
                    println!("Error getting tag for app '{}': << {} >>", app_struct.get_app_name(), e);
                    exit(1);
                }
            }
        }
        tags_hashmap
    }

    fn patch_app(&mut self, app_name: String) {
        let modpath_dir = &format!("{}/{}", self.config.get_db_dir_path(), app_name);
        let xrd_game_folder = self.config.get_xrd_game_folder().to_string();

        let mut app = self.config.apps.get_mut(&app_name).unwrap();
        match app.app_type {
            AppType::HitboxOverlay | AppType::FasterLoadingTimes | AppType::BackgroundGamepad => {
                match app.patch_app(xrd_game_folder, modpath_dir) {
                    Ok(_) => {app.patched=true;}
                    Err(e) => {println!("Error when patching app '{}' '{e}'",app.get_app_name())}
                }
            }
            _ => {println!("[🚫] App '{}' of type {:?} doesn't have a patch procedure. Skipping", app.get_app_name(),app.app_type)}
        }
    }

    fn update_app(&mut self, app_name: String, latest_tag_info: &TagInfo) {
        let db_dir_path = self.config.get_db_dir_path().to_string();
        let modpath_dir = &format!("{}/{}", db_dir_path, app_name);
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

        let mut app_to_update = self.config.apps.get_mut(&app_name).unwrap();

        // App update (download new files)
        if app_to_update.tag_name == latest_tag_info.tag_name.to_string() {
            println!("[✅ ] APP {} is up to date, skipping...", app_name);
        } else {
            println!("[⚠️ ] Updating '{}'", app_name);
            match app_to_update.app_type {
                AppType::HitboxOverlay | AppType::FasterLoadingTimes | AppType::WakeupTool | AppType::MirrorColorSelect | AppType::BackgroundGamepad  => {
                    app_to_update.download_mod(modpath_dir, latest_tag_info);
                }
                _ => {println!("[🚫] App '{}' of type {:?} doesn't have a update procedure. Skipping", app_name, app_to_update.app_type)}
            }
        }

        app_to_update.tag_name = latest_tag_info.tag_name.to_string();
        app_to_update.published_at = latest_tag_info.published_at.to_string();
        app_to_update.url_source_version = latest_tag_info.html_url.to_string();
        app_to_update.id = latest_tag_info.id;
    }

    pub(crate) fn update_all(&mut self){
        let tags_hashmap: HashMap<String, TagInfo> = self.get_latest_tags_hash_map();
        let mut new_verison_found_bool: bool = false;

        for (app_name,latest_tag_info) in &tags_hashmap {
            match self.config.apps.get(app_name) {
                Some(current_app)  => {
                    if new_verison_found_bool.bitor(print_different_versions(current_app,latest_tag_info)) {
                        new_verison_found_bool=true
                    }

                }
                None => {
                    println!("App '{}' not found. Skipping for tag with url '{}'",app_name,latest_tag_info.html_url);
                }
            }
        }

        // Download
        if !new_verison_found_bool {
            println!("No new versions found. Exiting...");
        }
        else {

            let ans = Confirm::new("Do you wish to update to the latest version?").
                with_default(false).
                with_help_message("This will update all the mentioned apps").
                prompt();

            match ans {
                Ok(true) => {
                    // Download
                    for (app_name,latest_tag_info) in &tags_hashmap {
                        self.update_app(app_name.to_owned(), latest_tag_info);
                    }

                    match self.save_config(){
                        Ok(_) => {
                            println!("Successfully saved the configuration.")
                        }
                        Err(e) => {
                            println!("Error Saving the configuration: '{}'",e)
                        }
                    }

                },
                Ok(false) => println!("That's too bad, I've heard great things about it."),
                Err(_) => println!("Error with the input."),
            }
        }

        // Patch
        /// Get which apps to patch
        let mut apps_to_patch_vec: Vec<String> = vec![];
        for app in self.config.apps.values_mut() {
            if app.automatically_patch && !app.patched {
                apps_to_patch_vec.push(app.get_app_name());
            }
        }

        // Patch the apps
        for app_name in apps_to_patch_vec {
            self.patch_app(app_name);
        }

        // Post patch save
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

// Functions

fn get_xrd_folder_from_file (steam_vdf_file_path: String) -> std::io::Result<String>  {
    let contents = fs::read_to_string(steam_vdf_file_path)?.replace("\t"," ");

    let mut xrd_line: i32=-1;
    let xrd_game_id_txt = "\"520440\"";

    let mut file_lines = contents.lines();
    let mut last_storage_path:String="".to_string();
    let mut current_line_count = 0;
    let mut current_line_string: String;

    while xrd_line < 0 && current_line_count < contents.lines().count() {
        current_line_string = file_lines.next().unwrap().to_string();

        if current_line_string.contains(xrd_game_id_txt)  {
            xrd_line = current_line_count as i32;
        }

        if current_line_string.contains("\"path\"") && xrd_line < 0 {
            let mut tmp_path: String = current_line_string;
            tmp_path = tmp_path.trim().to_string(); // remove extra spaces left right
            tmp_path = tmp_path.strip_prefix("\"path\"").unwrap().to_string(); // Remove starter "path"
            tmp_path = tmp_path.trim().to_string(); // Trim again
            tmp_path = tmp_path.replace("\"",""); // Remove quotes
            last_storage_path = tmp_path;
        }

        current_line_count +=1;
    }

    if xrd_line < 0 {
        println!("Xrd not found, exitting...");
        exit(1);
    }

    if cfg!(windows) {
        Ok(format!("{}\\steamapps\\common\\GUILTY GEAR Xrd -REVELATOR-",last_storage_path))
    } else {
        Ok(format!("{}/steamapps/common/GUILTY GEAR Xrd -REVELATOR-",last_storage_path))
    }
}

fn print_different_versions(current:&AppStruct, latest:&TagInfo) -> bool {
    // for convenience returns true if a new version is fouund.

    println!("Checking updates for app: {}",current.get_app_name());

    if current.tag_name == latest.tag_name && current.published_at == latest.published_at {
        println!("[✅ ] APP {} is up to date!",current.get_app_name());
        return false
    } else {
        println!("[⚠️ ] APP {} has a new version detected.",current.get_app_name());

        // Version
        println!("Version:\t'{}' -> '{}'",current.tag_name,latest.tag_name);
        // Published date
        println!("Published date: '{}' -> '{}'",current.published_at,latest.published_at);
        // Source URL
        println!("Source URL: '{}'",latest.html_url);
        // Print notes
        println!("Version notes:\n============\n{}\n============",latest.body.replace("\\n","\n").replace("\\r",""));
    }
    true
}

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
    // this was a copy pasta from somewhere

    let zipfile = File::open(&zip_file_path).unwrap();

    let mut archive = ZipArchive::new(zipfile).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = format!("{}/{}",unzip_dir,file.name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            // println!("File {} extracted to \"{}\"", i, outpath);
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // println!("File {} extracted to \"{}\" ({} bytes)",i,outpath,file.size());
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    println!("File '{}' extracted to '{}'",zip_file_path,unzip_dir);
}

