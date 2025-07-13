mod ratatui_app;
mod apps;

use std::io;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use itertools::Itertools;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::{TableState, Widget};
use ratatui_app::*;
use apps::*;
use ratatui::style::palette::tailwind::{SLATE};
use ratatui::text::Line;

fn main() -> io::Result<()>  {
    println!("hi");

    // Check if config exists
    let default_apps = crate::apps::get_default_apps();
    let mut exit:bool = false;

    // If it doesn't ask whether create a new one or not
    // Either leave or create a new one.

    // If it does continue
    let mut current_menu = MenuToRender::AppList;

    color_eyre::install();
    let mut terminal = ratatui::init();

    let mut mod_list_table = ModListTable {
        sort_ascend: false,
        state: TableState::default().with_selected(0),
        apps_hashmap: default_apps,
        colors: TableColors::new()
    };

    let mut app_menu_options = AppMenuOptions{ state: Default::default(), app: None, colors: TableColors::new() };

    while !exit {
        match current_menu {
            MenuToRender::AppList => {terminal.draw(|frame| mod_list_table.render(frame))?;}
            MenuToRender::AppMenuOptions => {terminal.draw(|frame| app_menu_options.render(frame))?;}
        }

        if let Event::Key(key) = event::read()? {
            match current_menu {
                MenuToRender::AppList => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => exit=true,
                            KeyCode::Up => mod_list_table.select_previous(),
                            KeyCode::Down => mod_list_table.select_next(),
                            KeyCode::Enter => {
                                app_menu_options = AppMenuOptions {
                                    state: Default::default(),
                                    app: mod_list_table.get_selected_app(),
                                    colors: TableColors::new(),
                                };
                                current_menu=MenuToRender::AppMenuOptions;
                            },
                            KeyCode::Char('r') | KeyCode::Char('R') => mod_list_table.sort_ascend=!mod_list_table.sort_ascend,
                            _ => {}
                        }
                    }
                }

                MenuToRender::AppMenuOptions => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => current_menu=MenuToRender::AppList,
                            KeyCode::Up => app_menu_options.select_previous(),
                            KeyCode::Down => app_menu_options.select_next(),
                            KeyCode::Enter => {
                                // current_menu=MenuToRender::AppMenuOptions;
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