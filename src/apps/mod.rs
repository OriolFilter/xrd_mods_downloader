use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum AppType {
    #[default]
    Unknown,
    HitboxOverlay,
    WakeupTool,
    FasterLoadingTimes,
    MirrorColorSelect,
    BackgroundGamepad
}


#[derive(Default, Serialize, Deserialize,Debug, Clone)]
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

    pub(crate) fn new(repo_owner: String, repo_name: String, app_type: AppType) -> Self {
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
        format!("{}/{}", self.repo_owner, self.repo_name).to_string()
    }

    fn get_repo_url(&self) -> String {
        format!("https://github.com/{}/{}", self.repo_owner, self.repo_name).to_string()
    }
    fn get_api_repo_url(&self) -> String {
        format!("https://api.github.com/repos/{}/{}", self.repo_owner, self.repo_name).to_string()
    }
}


pub(crate) fn get_default_apps ()  -> HashMap<String,AppStruct> {
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

    new_app_hashmap
    // self.apps = new_app_hashmap;
}