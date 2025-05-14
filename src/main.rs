mod stuff;
use stuff::*;

// mod my_ratatui_app;
mod actual_new_ratatui_app;
// use my_ratatui_app::*;

// mod new_my_ratatui_app;

mod manager;
use manager::*;
mod functions;


use color_eyre::Result;
use futures::Stream;
use ratatui::style::Stylize;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write as StdinWrite;
use std::io::{Read, Seek, Write};
use std::ops::BitOr;


// Linux imports
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

// Windows imports
#[cfg(target_os = "windows")]
// Get path from Windows registry
use winreg::{enums::*, RegKey};

fn main() -> Result<()> {
    // let mut manager = Manager::default();
    //
    // match manager.load_config() {
    //     Ok(_) => {println!("Config loaded correctly")}
    //     Err(e) => {println!("There was an error loading the config: {e}")}
    // }
    //
    // println!("Xrd folder located at: '{}'",manager.config.get_xrd_game_folder());
    //
    // manager.update_all();
    // Ok(())

    // let _ = Confirm::new("Done").
    //     with_default(true).
    //     with_help_message("Press enter to exit...").prompt();
    println!("hi");
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = crate::actual_new_ratatui_app::App::default().run(terminal);
    ratatui::restore();
    println!("bye");
    app_result
}

