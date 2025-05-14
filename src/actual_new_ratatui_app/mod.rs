//! # [Ratatui] Tabs example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::collections::hash_map::Keys;
use std::net::ToSocketAddrs;
use std::thread;
use std::thread::sleep;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use dirs::config_dir;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
    DefaultTerminal,
};
use serde::{Deserialize, Serialize};

use ratatui::{
    style::{Style},
    widgets::{List, ListState},
    Frame,
};
use ratatui::prelude::StatefulWidget;
use ratatui::style::palette::tailwind::{GREEN, SLATE};
use ratatui::widgets::{HighlightSpacing, ListItem, Wrap};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::manager::Manager;
use crate::stuff;
use crate::stuff::AppStruct;


//

#[derive(Default)]
struct AppStructListMenu {
    apps: Vec<AppStruct>,
    state: ListState
}

#[derive(Debug)]
struct TodoItem {
    todo: String,
    info: String,
    status: Status,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Todo,
    Completed,
}



// Consts

const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

#[derive(Default)]
pub struct App {
    state: AppState,
    selected_tab: SelectedTab,
    config_manager: Manager,
    app_struct_list_menu: AppStructListMenu,
    list_state: ListState
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Manage Mods")]
    Tab1,
    #[strum(to_string = "Tab 2")]
    Tab2,
    #[strum(to_string = "Patch Mods")]
    Tab3,
    #[strum(to_string = "Tab 4")]
    Tab4,
}


impl App {
    pub(crate) fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.config_manager.load_config();
        // let mut widget_list_state = ListState::default();

        while self.state == AppState::Running {
            terminal.draw(|frame|
                frame.render_widget(&mut self, frame.area())
                // frame.render_stateful_widget(&mut self, frame.area(), &mut widget_list_state)
                // frame.render_stateful_widget(&self, frame.area(), &mut widged_state)
            )?;
            // for ter
            // for x in terminal.draw() {}

            self.handle_events()?;


        }

        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match self.selected_tab {
                    SelectedTab::Tab1 => {
                        match key.code {
                            // Tab specific

                            KeyCode::Char('s') | KeyCode::Char('S') => { self.save_config() }
                            KeyCode::Char('r') | KeyCode::Char('R')=> { self.reload_config() }

                            // Movement
                            KeyCode::Enter => { self.enable_disable_mod()}
                            KeyCode::Up => { self.select_previous() }
                            KeyCode::Down => { self.select_next() }

                            // Tab Movement

                            // Others
                            KeyCode::Char('q') | KeyCode::Char('Q')| KeyCode::Esc => self.quit(),
                            _ => {}
                        }
                    }
                    // SelectedTab::Tab2 => {}
                    // SelectedTab::Tab3 => {}
                    // SelectedTab::Tab4 => {}
                    _ => {
                        match key.code {
                            // KeyCode::Right => self.next_tab(),
                            // KeyCode::Left => self.previous_tab(),
                            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => self.quit(),
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }
    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    fn select_first(&mut self) {
        self.list_state.select_first();
    }

    fn select_last(&mut self) {
        self.list_state.select_last();
    }

    // Tabs
    // pub fn next_tab(&mut self) {
    //     self.selected_tab = self.selected_tab.next();
    // }
    //
    // pub fn previous_tab(&mut self) {
    //     self.selected_tab = self.selected_tab.previous();
    // }


    // State
    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }

    pub fn reload_config(&mut self) {
        self.config_manager=Manager::default();
        self.config_manager.load_config();
    }

    pub fn save_config(&mut self) {
        self.config_manager.save_config();
    }


    // Tab 1
    fn enable_disable_mod(&mut self) {
        match self.list_state.selected() {
            Some(index) => {
                let app_list = self.config_manager.get_sorted_apps_string();
                let app = self.config_manager.config.apps.get_mut(app_list.get(index).unwrap());
                app.unwrap().enabled ^= true;
            }
            _ => {}
        }


        // use thread::sleep_ms;
        // sleep_ms(111111111);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Tab1 => tailwind::BLUE,
            Self::Tab2 => tailwind::EMERALD,
            Self::Tab3 => tailwind::INDIGO,
            Self::Tab4 => tailwind::RED,
        }
    }
}

// impl StatefulWidget for &mut App {
//     type State = ListState;

impl Widget for &mut App {
    // fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Get range and thingies.
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);


        match self.selected_tab {
            SelectedTab::Tab1 => self.selected_tab.enable_mods_tab(inner_area, buf, &self.config_manager, &mut self.list_state),
            _ => { println!("tab out of bounds!") }
        }

        render_footer(self,footer_area,buf);

    }
}


impl SelectedTab {

    fn enable_mods_tab(self, area: Rect, buffer: &mut Buffer, manager: &Manager, list_state: &mut ListState) {
        // let mut app_list: Vec<String> = vec![];
        // for app in manager.config.apps.keys() {
        //     app_list.push(app.to_string());
        // }
        let mut app_list: Vec<String> = manager.config.apps.iter().map(|(app_name,app)| {app.get_app_name()}).collect();
        app_list.sort();

        let mut c=0;
        let items_list: Vec<ListItem> = app_list
            .iter()
            .map(|app_name| {
                let color = alternate_colors(c);
                c+=1;
                ListItem::from(manager.config.apps.get(app_name).unwrap()).bg(color)
            })
            .collect();

        let list = List::new(items_list)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);


        StatefulWidget::render(list, area, buffer, list_state);
        // Widget::render(list, area, buffer);
        // println!()
    }

}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Ratatui Tabs Example".bold().render(area, buf);
}

fn render_footer(app: &App, area: Rect, buf: &mut Buffer) {
    match app.selected_tab {
        SelectedTab::Tab1 => {
            Line::raw("Use ↓↑ to move | ◄ ► to change tab | S/s to save | Q/q to quit")
                .centered()
                .render(area, buf);
        }
        // SelectedTab::Tab2 => {}
        // SelectedTab::Tab3 => {}
        // SelectedTab::Tab4 => {}
        _ => {
            Line::raw("◄ ► to change tab | Press q to quit")
                .centered()
                .render(area, buf);
        }
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&AppStruct> for ListItem<'_> {
    fn from(app: &AppStruct) -> Self {
        let line = match app.enabled {
            true => Line::styled(format!(" ✓ {}", app.get_app_name()), COMPLETED_TEXT_FG_COLOR),
            false => Line::styled(format!(" ☐ {}", app.get_app_name()), TEXT_FG_COLOR)
        };
        ListItem::new(line)
    }
}

impl From<AppStruct> for ListItem<'_> {
    fn from(value: AppStruct) -> Self {
        let line = match value.enabled {
            false => Line::styled(format!(" ☐ {}", value.repo_name), TEXT_FG_COLOR),
            true => {
                Line::styled(format!(" ✓ {}", value.repo_owner), COMPLETED_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}