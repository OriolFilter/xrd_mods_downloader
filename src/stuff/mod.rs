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
use crate::functions::{download_file_to_path, get_xrd_folder_from_file, unzip_file};

// Linux imports
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;
use strum::{Display, EnumIter, FromRepr};
// Windows imports
#[cfg(target_os = "windows")]
// Get path from Windows registry
use winreg::{RegKey,enums::*};

#[derive(Serialize, Deserialize, Debug)]
pub struct TagAssets {
    // url: String,
    id: i32,
    pub name: String,
    content_type: String,
    state: String,
    size: i32,
    pub browser_download_url: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TagInfo {
    // url: String,
    pub(crate) html_url: String,
    pub(crate) id: i32,
    pub(crate) tag_name: String,
    tarball_url: String,
    body: String,
    pub(crate) published_at: String,
    assets: Vec<TagAssets>
}

impl TagInfo {
    pub(crate) fn get_formated_body(&self) -> String {
        self.body.replace("\\r", "")
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum AppType {
    #[default]
    Unknown,
    HitboxOverlay,
    WakeupTool,
    FasterLoadingTimes,
    MirrorColorSelect,
    BackgroundGamepad
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct AppStruct {
    pub(crate) repo_owner: String,
    pub(crate) repo_name: String,
    // App type identifier
    #[serde(default)]
    pub(crate) app_type: AppType,
    // To update with each version
    #[serde(default)]
    pub(crate) id: i32,
    #[serde(default)]
    pub(crate) tag_name: String,
    #[serde(default)]
    pub(crate) published_at: String,
    #[serde(default)]
    pub(crate) url_source_version: String,
    #[serde(default = "set_false")]
    pub(crate) automatically_patch: bool,
    #[serde(default = "set_false")]
    pub(crate) patched: bool,
    #[serde(default = "set_false")]
    pub(crate) enabled: bool, // Whether if you want to keep it track/visible in other windows or not
    #[serde(default = "set_false")] // TODO Unused
    pub(crate) track_updates: bool,
    #[serde(default = "set_false")] // TODO Unused
    pub(crate) tracked: bool,

}


fn set_false() -> bool {
    false
}


impl AppStruct {

    // TODO
    // Return whether if it's possible or not to launch the mod.
    pub(crate) fn is_launchable() -> bool {
        false
    }

    pub(crate) fn new(repo_owner: String, repo_name:String, app_type:AppType) -> Self {
        Self {
            repo_owner: repo_owner,
            repo_name: repo_name,
            id: 0,
            tag_name: "".to_string(),
            published_at: "".to_string(),
            app_type: app_type,
            url_source_version: "".to_string(),
            automatically_patch: false,
            patched: false,
            enabled: false,
            track_updates: false,
            tracked: false,
        }
    }
    pub(crate) fn get_app_name(&self) -> String {
        format!("{}/{}",self.repo_owner,self.repo_name).to_string()
    }

    fn get_repo_url(&self) -> String{
        format!("https://github.com/{}/{}",self.repo_owner,self.repo_name).to_string()
    }
    fn get_api_repo_url(&self) -> String{
        format!("https://api.github.com/repos/{}/{}",self.repo_owner,self.repo_name).to_string()
    }

    pub(crate) fn download_mod(&self, destination_dir: &String, tag_info: &TagInfo) -> std::io::Result<()> {
        let mut assets_whitelist:Vec<String> = vec![];

        match self.app_type {
            AppType::WakeupTool => {
                assets_whitelist = vec![
                    format!("GGXrdReversalTool.{}.zip",tag_info.tag_name), // Iquis
                    format!("GGXrdReversalTool-{}.zip",tag_info.tag_name) // kkots
                ];
            }
            AppType::HitboxOverlay => {
                assets_whitelist = vec!["ggxrd_hitbox_overlay.zip".to_string()];
            }
            AppType::FasterLoadingTimes => {
                if cfg!(windows) {
                    assets_whitelist = vec!["GGXrdFasterLoadingTimes.exe".to_string()];
                }
                else if cfg!(unix) {
                    assets_whitelist = vec!["GGXrdFasterLoadingTimes_linux".to_string()];
                }
                else {
                    // println!("Neither Linux or Windows detected, skipping tag {}",tag_info.tag_name);
                }
            }
            AppType::MirrorColorSelect => {
                assets_whitelist = vec!["GGXrdMirrorColorSelect.zip".to_string()];
            }
            AppType::BackgroundGamepad => {
                if cfg!(windows) {
                    assets_whitelist = vec!["GGXrdBackgroundGamepad.exe".to_string()];
                }
                else if cfg!(unix) {
                    assets_whitelist = vec!["GGXrdBackgroundGamepad_linux".to_string()];
                }
                else {
                    // println!("Neither Linux or Windows detected, skipping tag {}",tag_info.tag_name);
                }
            }

            AppType::Unknown | _ => {}
        }

        let mut matched_assets_list: Vec<&TagAssets> = vec![];

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

        // Todo raise error
        Ok(())
    }

    pub(crate) fn patch_app(&self, xrd_game_folder: String, downloaded_mod_folder: &String) -> io::Result<()> {
        // This assumes that only Linux or Windows will reach this point.
        let xrd_binaries_folder_path = format!("{}/Binaries/Win32", xrd_game_folder);
        let mut files_to_copy:Vec<String> = vec![]; // files to only copy
        let mut file_to_execute:String = String::new(); // file to execute. Copy skipped
        let mut successful_patch =false;
        let mut executable_filepath: String = String::new();

        // prepare patch
        match self.app_type {
            AppType::HitboxOverlay => {
                files_to_copy = vec![
                    "ggxrd_hitbox_overlay.dll".to_string(),
                ];

                if cfg!(windows){ file_to_execute = "ggxrd_hitbox_patcher.exe".to_string();}
                else if cfg!(unix) {file_to_execute = "ggxrd_hitbox_patcher_linux".to_string();}

            }
            AppType::FasterLoadingTimes => {

                if cfg!(windows){ file_to_execute = "GGXrdFasterLoadingTimes.exe".to_string();}
                else if cfg!(unix) {file_to_execute = "GGXrdFasterLoadingTimes_linux".to_string();}

            }
            AppType::BackgroundGamepad => {

                if cfg!(windows){ file_to_execute = "GGXrdBackgroundGamepad.exe".to_string();}
                else if cfg!(unix) {file_to_execute = "GGXrdBackgroundGamepad_linux".to_string();}

            }
            AppType::Unknown | _ => {}
        }

        for filename in files_to_copy {
            // Copy from local_mod_folder to xrd_game_folder
            let source_file_path = format!("{}/{}", downloaded_mod_folder, filename);
            let destination_new_file_path = format!("{}/{}", xrd_binaries_folder_path, filename);
            match fs::copy(source_file_path.to_string(), destination_new_file_path.to_string()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error copying '{}' -> '{}' <{e}>.", source_file_path,destination_new_file_path);
                }
            }
        }

        if !file_to_execute.is_empty() {
            executable_filepath = format!("{}/{}", downloaded_mod_folder, file_to_execute);

            // set chmod +x permissions (linux)
            #[cfg(target_os = "linux")]
            {
                let permissions = Permissions::from_mode(0o755);
                fs::set_permissions(executable_filepath.to_string(),permissions)?;
            }

            // Call command
            println!("Executing {}",executable_filepath);

            let mut command = Command::new(executable_filepath.to_string());
            let mut stdin_input: String = String::new();
            let mut child = command.stdin(Stdio::piped())
                .spawn()?;
            let mut stdin_pipe = child.stdin.take().unwrap();

            // Stdin
            if cfg!(unix) {

                stdin_input = String::new();

                // Stdin (Custom per app)
                match self.app_type {
                    AppType::HitboxOverlay | AppType::FasterLoadingTimes | AppType::BackgroundGamepad => {
                        stdin_input =format!("\n{xrd_binaries_folder_path}/GuiltyGearXrd.exe\n\n");
                        stdin_pipe.write_all(stdin_input.as_bytes()).unwrap();
                    }
                    AppType::Unknown => {
                        println!("App {} of type {:?} doesn't support patching on Linux",self.get_app_name(),self.app_type);
                    }
                    // Some apps might not require stdin
                    _ => {}
                }
            }

            // Stdout
            println!("==============\n=== Stdout ===\n==============");
            let mut child_wait = child.wait_with_output();

            // Check status
            println!("==============\n=== Stderr ===\n==============");
            let child_wait_unwrap = child_wait.unwrap();

            match child_wait_unwrap.status.code() {
                Some(0) => {
                    successful_patch = true;
                }

                Some(-1073741701) => { //x86
                    println!("Exit code '{}'. Some DLLs might be missing.\nRefer to here to install the Latest Microsoft Visual C++ Redistributable Version https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#latest-microsoft-visual-c-redistributable-version",child_wait_unwrap.status.code().unwrap(),)
                }
                Some(-1073741515) => { //x64
                    println!("Exit code '{}'. Some 64bit DLLs might be missing.\nRefer to here to install the Latest Microsoft Visual C++ Redistributable Version https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#latest-microsoft-visual-c-redistributable-version",child_wait_unwrap.status.code().unwrap(),)
                }
                unknown_code => {
                    println!("Exit code '{}'. Ensure that the mod's executables can be manually executed. Maybe DLL are missing, for 32/86bits and/or 64bits https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#latest-microsoft-visual-c-redistributable-version",unknown_code.unwrap())
                }
            }
        }
        else { println!("App '{}' with type '{:?}' has no executable files declared, skipping patch",self.get_app_name(), self.app_type); }
        if successful_patch {
            Ok(())
        } else {
            panic!("Error while executing {}.",executable_filepath)
        }
    }

    #[tokio::main]
    pub(crate) async fn get_latest_tag(&self) -> color_eyre::Result<TagInfo, reqwest::Error> {
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
        let mut tag_info: TagInfo = TagInfo {
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
                // println!("Unknown error. Status code {} when getting the latest tag for the repository {}",other, self.get_api_repo_url());
            }
        }

        Ok(tag_info)
    }

}




#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub(crate) apps: HashMap<String,AppStruct>,
    #[serde(default)]
    pub(crate) xrd_game_folder: String
}

impl Config {
    pub(crate) fn set_default_apps (&mut self) {
        let mut new_app_hashmap: HashMap<String,AppStruct> = HashMap::new();
        let mut holder_apps_vector: Vec<AppStruct> = vec![];
        // holder_apps_vector.push(AppStruct::new("a".to_string(),"a".to_string(), AppType::HitboxOverlay));
        // holder_apps_vector.push(AppStruct::new("b".to_string(),"b".to_string(), AppType::HitboxOverlay));
        // holder_apps_vector.push(AppStruct::new("c".to_string(),"c".to_string(), AppType::HitboxOverlay));


        // Hitbox Overlay
        holder_apps_vector.push(AppStruct::new("kkots".to_string(),"ggxrd_hitbox_overlay_2211".to_string(), AppType::HitboxOverlay));

        // Wake up tool Iquis
        holder_apps_vector.push(AppStruct::new("Iquis".to_string(),"rev2-wakeup-tool".to_string(), AppType::WakeupTool));

        // // Wake up tool kkots
        holder_apps_vector.push(AppStruct::new("kkots".to_string(),"rev2-wakeup-tool".to_string(), AppType::WakeupTool));

        // // Faster Loading Times kkots
        holder_apps_vector.push(AppStruct::new("kkots".to_string(),"GGXrdFasterLoadingTimes".to_string(), AppType::FasterLoadingTimes));

        // // Mirror Color Select kkots
        holder_apps_vector.push(AppStruct::new("kkots".to_string(),"GGXrdMirrorColorSelect".to_string(), AppType::MirrorColorSelect));

        // // Background Gamepad kkots
        holder_apps_vector.push(AppStruct::new("kkots".to_string(),"GGXrdBackgroundGamepad".to_string(), AppType::BackgroundGamepad));

        for app in holder_apps_vector {
            new_app_hashmap.insert(app.get_app_name(),app);
        }

        self.apps = new_app_hashmap;
    }

    pub(crate) fn get_db_dir_path(&mut self) -> String {
        match env::var("XRD_MOD_FOLDER") {
            Ok(env_val) => {
                // println!("XRD_MOD_FOLDER env is set to: {}. Overwriting executable location.",env_val);
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
    pub(crate) fn get_db_file_path(&mut self) -> String {
        format!("{}/{}", self.get_db_dir_path(), "db.json")
    }

    pub(crate) fn get_xrd_game_folder(&mut self) -> String {
        if self.xrd_game_folder.is_empty() {
            let mut file_path: String=String::new();

            if cfg!(windows) {
                let mut steampath=String::new();

                // TODO improve

                #[cfg(target_os = "windows")]
                {
                    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                    let cur_ver = hklm.open_subkey("SOFTWARE\\Wow6432Node\\Valve\\Steam").unwrap();
                    steampath = cur_ver.get_value("InstallPath").unwrap();
                }

                file_path = format!("{steampath}\\config\\libraryfolders.vdf").to_string();
            }
            else if cfg!(unix) {
                let home_path = dirs::home_dir().unwrap().to_str().unwrap().to_string();
                file_path = format!("{home_path}/.steam/root/config/libraryfolders.vdf").to_string();

            }
            else {
                println!("Neither Linux or Windows detected, skipping tag.");
                exit(1);
            }
            self.xrd_game_folder = get_xrd_folder_from_file(file_path.to_string()).unwrap().to_string();
        }
        self.xrd_game_folder.to_string()
    }
}

