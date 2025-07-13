mod ratatui_app;
mod apps;
mod functions;
mod download_manager;

use std::io;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use futures::task::Spawn;
use itertools::Itertools;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::{TableState, Widget};
use ratatui_app::*;
use apps::*;
use ratatui::style::palette::tailwind::{SLATE};
use ratatui::text::Line;
use download_manager::update_app_async;
use ratatui_app::AppMenuOptionsList;
use functions::{launch_mod};
use crate::download_manager::{AppUpdateManager, AppUpdateManagerStatus, ModUpdatingStatus};
use tokio::spawn;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> io::Result<()>  {
    println!("hi");

    // Check if config exists
    let default_apps = crate::apps::get_default_apps();
    let mut exit:bool = false;

    // If it doesn't ask whether create a new one or not
    // Either leave or create a new one.

    // If it does continue
    let mut menu_to_render = MenuToRender::AppList;

    color_eyre::install();
    let mut terminal = ratatui::init();

    let mut mod_list_table = ModListTable {
        sort_ascend: false,
        state: TableState::default().with_selected(0),
        apps_hashmap: default_apps,
        colors: TableColors::new()
    };

    let mut app_menu_options = AppMenuOptions{ state: Default::default(), app: None, colors: TableColors::new() };
    let mut update_manager_spawn: Option<JoinHandle<()>> = None;
    // let mut update_manager_spawn: Option<JoinHandle<()>>;
    let mut app_update_manager = crate::download_manager::AppUpdateManager::new();
    while !exit {
        // if update_manager_spawn.is_some() {
        // } else {
        match app_update_manager.get_status() {
            AppUpdateManagerStatus::Running => {}
            AppUpdateManagerStatus::Finished => {}
            AppUpdateManagerStatus::Pending => {
                match menu_to_render {
                    MenuToRender::AppList => {terminal.draw(|frame| mod_list_table.render(frame))?;}
                    MenuToRender::AppMenuOptions => {terminal.draw(|frame| app_menu_options.render(frame))?;}
                }
            }
        }
        // // If something means it's updating
        // //         match handle.is_finished() {
        // //             true => {}
        // //             false => {}
        // //         }
        //
        //         // Check which is the state and render the appropriate window, aka, weather if it's done or not.
        //
        //     }
            // None => {
            //     // If empty means not updating
            // }
        // }


        if let Event::Key(key) = event::read()? {
            match menu_to_render {
                MenuToRender::AppList => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => exit=true,
                            KeyCode::Up => mod_list_table.select_previous(),
                            KeyCode::Down => mod_list_table.select_next(),
                            KeyCode::Enter => {
                                app_menu_options = AppMenuOptions {
                                    state: Default::default(),
                                    app: mod_list_table.get_selected_app().cloned(),
                                    colors: TableColors::new(),
                                };
                                menu_to_render =MenuToRender::AppMenuOptions;
                            },
                            KeyCode::Char('r') | KeyCode::Char('R') => mod_list_table.sort_ascend=!mod_list_table.sort_ascend,
                            _ => {}
                        }
                    }
                }

                MenuToRender::AppMenuOptions => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => menu_to_render =MenuToRender::AppList,
                            KeyCode::Up => app_menu_options.select_previous(),
                            KeyCode::Down => app_menu_options.select_next(),
                            KeyCode::Enter => {
                                match app_menu_options.get_selected_menu().clone() {
                                    None => {}
                                    Some(selected_app_menu) => {
                                        match selected_app_menu {
                                            AppMenuOptionsList::Launch => {
                                                launch_mod(mod_list_table.get_selected_app().unwrap())?;
                                            }

                                            AppMenuOptionsList::Download => {
                                                app_update_manager = AppUpdateManager::new();
                                                app_update_manager.add_app_to_update(app_menu_options.app.clone().unwrap());
                                                app_update_manager.async_spawn= Option::from(spawn(app_update_manager.update_apps()));
                                                // update_manager_spawn = Some(spawn(app_update_manager.update_apps()));
                                                // app_update_manager.start_async_update()?;
                                                // let x= update_app_async(vec![app_menu_options.app.clone().unwrap()]);
                                                // update_manager_spawn = Some(spawn(update_app_async(vec![app_menu_options.app.clone().unwrap()])));
                                            }

                                            AppMenuOptionsList::Patch => {} //
                                            AppMenuOptionsList::Update => {} // Same as Download :shrug:
                                            AppMenuOptionsList::Description => {}, // Maybe don't render/do anything and display this directly while in the app menu page.
                                        }
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    };

    ratatui::restore();

    println!("bye");
    Ok(())
    // app_result
}