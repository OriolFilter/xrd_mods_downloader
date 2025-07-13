use std::io;
use tokio::spawn;
use tokio::task::JoinHandle;
use crate::apps::AppStruct;

#[derive(Default)]
enum AppUpdatingStatusStatus {
    #[default]
    Pending,
    OnGoing,
    Updated,
    Failed
}

pub(crate) struct ModUpdatingStatus {
    pub(crate) app: AppStruct,
    pub(crate) status: AppUpdatingStatusStatus
    // Probably make a vector of errors, or a hashmap to separate them by phases.
}

impl ModUpdatingStatus {
    fn get_status_string(&self) -> String {
        format!("{} ({})",self.app.get_app_name(),
                match self.status {
                    AppUpdatingStatusStatus::Pending => {"Pending"}
                    AppUpdatingStatusStatus::OnGoing => {"On Going"}
                    AppUpdatingStatusStatus::Updated => {"Updated"}
                    AppUpdatingStatusStatus::Failed  => {"Failed"}
                }).to_string()
    }
    // fn get_status_render_colour(&self) -> Color {
    //     match self.status {
    //         AppUpdatingStatusStatus::Pending => GREY_TEXT_FG_COLOR,
    //         AppUpdatingStatusStatus::OnGoing => YELLOW_TEXT_FG_COLOR,
    //         AppUpdatingStatusStatus::Updated => COMPLETED_TEXT_FG_COLOR,
    //         AppUpdatingStatusStatus::Failed  => RED_TEXT_FG_COLOR
    //     }
    // }
}

#[derive(Default)]
#[derive(Clone)]
pub(crate) enum AppUpdateManagerStatus {
    #[default]
    Pending,
    Running,
    Finished
}

pub(crate) struct AppUpdateManager {
    pub(crate) apps_to_update: Vec<ModUpdatingStatus>, // TODO make it a hashmap so there are no dupes (for whatever reason).
    pub(crate) status: AppUpdateManagerStatus,
    pub(crate) async_spawn: Option<JoinHandle<()>>
}

impl AppUpdateManager {
    pub(crate) fn new() -> Self {
        Self {
            apps_to_update: vec![],
            status: Default::default(),
            async_spawn: Default::default(),
        }
    }
}

impl AppUpdateManager {
    pub(crate) fn add_app_to_update(&mut self, app:  AppStruct) {
        self.apps_to_update.push(ModUpdatingStatus{ app, status: AppUpdatingStatusStatus::Pending })
    }

    pub(crate) fn quit(&mut self) {
        // idk cleanup/stop whatever it's doing something.
    }

    // pub(crate) fn update_app(&mut self, app: &AppStruct) {
    pub(crate) async fn update_apps(&mut self) {
    // async fn update_apps(&mut self, apps_to_update: Vec<AppStruct> ) {
        // Set running
        // Download if dont exists

        // Update if already exists
        // The pipeline should feel the same.
        // Downloading is like updating from Null to 'XYZ'

        self.status = AppUpdateManagerStatus::Running;
        loop {
            println!("Updating {:#?}", self.apps_to_update[0].app.get_app_name());
            std::thread::sleep_ms(1000);
        }

        // self.status = AppUpdateManagerStatus::Finished;
    }

    // pub(crate) fn start_async_update(&mut self) -> io::Result<()>  {
    //     let async_spawn = spawn(self.update_apps());
    //     self.async_spawn = Some(async_spawn);
    //     Ok(())
    // }
    pub(crate) fn get_status(&mut self) -> AppUpdateManagerStatus {
        self.status.to_owned()
    }
}

pub(crate) async fn update_app_async(apps_to_update: Vec<AppStruct> ) {
    // Set running
    // self.status = AppUpdateManagerStatus::Running;
    // Download if dont exists

    // Update if already exists
    // The pipeline should feel the same.
    // Downloading is like updating from Null to 'XYZ'

    loop {
        println!("Updating {:#?}", apps_to_update[0].get_app_name());
        std::thread::sleep_ms(1000);
    }
}
