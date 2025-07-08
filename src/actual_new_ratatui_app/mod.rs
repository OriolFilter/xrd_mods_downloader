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
use std::collections::HashMap;
use std::env::remove_var;
use std::net::ToSocketAddrs;
use std::ops::Index;
use std::process::exit;
use std::thread;
use std::thread::{sleep, sleep_ms, JoinHandle};
use std::time::Duration;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use dirs::config_dir;
use ratatui::{buffer::Buffer, crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Rect}, style::{palette::tailwind, Color, Stylize}, symbols, text, text::{Line, Text}, widgets::{Block, Padding, Paragraph, Tabs, Widget}, DefaultTerminal};
use serde::{Deserialize, Serialize};

use ratatui::{
    style::{Style},
    widgets::{List, ListState},
    Frame,
};
use ratatui::prelude::StatefulWidget;
use ratatui::style::palette::material::{RED, YELLOW};
use ratatui::style::palette::tailwind::{GREEN, SLATE, STONE};
use ratatui::widgets::{HighlightSpacing, ListItem, Wrap};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use crate::functions::print_different_versions;
use crate::manager::Manager;
use crate::stuff;
use crate::stuff::{AppStruct, TagInfo};

use derive_setters::Setters;
use itertools::Itertools;
use lipsum::lipsum;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::{Borders, Clear},
    Terminal,
};
use ratatui::style::Color::Red;
use ratatui::symbols::line;
use serde::de::IntoDeserializer;
use serde_json::to_string;
use tokio::task;

#[derive(Default)]
enum SubMenus {
    #[default]
    None,
    UpdateSingleApps,
    UpdateAllApps,
    UpdateAllCompleted
}


#[derive(Default)]
struct AppStructListMenu {
    apps: Vec<AppStruct>,
    state: ListState
}

#[derive(Default)]
enum AppUpdatingStatusStatus {
    #[default]
    Pending,
    OnGoing,
    Updated,
    Failed
}

struct AppUpdatingStatus {
    app_name: String,
    status: AppUpdatingStatusStatus
}

impl AppUpdatingStatus {
    fn get_status_string(&self) -> String {
        format!("{} ({})",self.app_name,
                match self.status {
                    AppUpdatingStatusStatus::Pending => {"Pending"}
                    AppUpdatingStatusStatus::OnGoing => {"On Going"}
                    AppUpdatingStatusStatus::Updated => {"Updated"}
                    AppUpdatingStatusStatus::Failed  => {"Failed"}
                }).to_string()
    }
    fn get_status_render_colour(&self) -> Color {
        match self.status {
            AppUpdatingStatusStatus::Pending => GREY_TEXT_FG_COLOR,
            AppUpdatingStatusStatus::OnGoing => YELLOW_TEXT_FG_COLOR,
            AppUpdatingStatusStatus::Updated => COMPLETED_TEXT_FG_COLOR,
            AppUpdatingStatusStatus::Failed  => RED_TEXT_FG_COLOR
        }
    }


}


// Consts
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;
const GREY_TEXT_FG_COLOR: Color = SLATE.c300;
const YELLOW_TEXT_FG_COLOR: Color = YELLOW.c200 ;
const RED_TEXT_FG_COLOR: Color = RED.a700 ;

#[derive(Default)]
pub struct App {
    running_state: AppState,
    selected_tab: SelectedTab,
    current_sub_menu: SubMenus,
    // config_manager: Manager, // Tabs shouldn't use this. Used to populate tabs/pivot point.
    app_struct_list_menu: AppStructListMenu,
    active_tab_storage: TabStorage,
    latest_pulled_tags_hashmap: HashMap<String,TagInfo>,
    update_apps_status_hashmap: HashMap<String, AppUpdatingStatus>,
    lock_inputs: bool,
    n: i32,
    stored_thread: Option<JoinHandle<()>>
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
    #[strum(to_string = "Download/Update mods")]
    Tab2,
    #[strum(to_string = "Patch Mods")]
    Tab3,
    #[strum(to_string = "Tab 4")]
    Tab4,
}

impl SelectedTab {
    pub(crate) fn describe_selected_mod_tag_description(self, area: Rect, buffer: &mut Buffer, tab_storage: &mut TabStorage, latest_tags_pulled_map: &mut HashMap<String,TagInfo>) {

        let create_block = |title: String| Block::bordered().gray().title(title.bold());
        let mut text_lines: Vec<Line>= vec![];
        let mut paragraph: Paragraph;
        let mut text: Text;

        match tab_storage.list_state.selected() {
            Some(index) => {
                let app_name_list = tab_storage.get_enabled_app_names(); // self.config_manager.get_sorted_apps_name();
                let app_name = app_name_list.get(index).unwrap().to_string();
                let app = tab_storage.config_manager.config.apps.get(&app_name).unwrap();
                match latest_tags_pulled_map.get(&app.get_app_name()) {
                    None => {
                        text_lines.push(Line::styled("No version found. Search for updates.".to_string(), YELLOW_TEXT_FG_COLOR));
                        text = Text::from(text_lines);
                        paragraph = Paragraph::new(text).gray().block(create_block(format!("{} '{}' -> '??'", app.get_app_name(), app.tag_name))).wrap(Wrap { trim: true });
                    }

                    Some(tag) => {
                        // for line in tag.get_formated_body().to_string().split("\n") {
                        //     text_lines.push(Line::styled(format!("{}", line.to_string()), COMPLETED_TEXT_FG_COLOR));
                        // }
                        // text = Text::from(tag.get_formated_body()).style(COMPLETED_TEXT_FG_COLOR);
                        text = Text::from(tag.get_formated_body().to_string()).style(COMPLETED_TEXT_FG_COLOR);
                        paragraph = Paragraph::new(text).gray().block(create_block(format!("{} '{}' -> '{}'", app.get_app_name(), app.tag_name, tag.tag_name.to_string()))).wrap(Wrap { trim: false });

                        // println!("{}", tag.get_formated_body());
                        // println!("{:?}", tag.get_formated_body());
                        // println!("{}", tag.get_formated_body().replace("\\n","\n").replace("\r","").replace("Adds", "CAAAALVO"));
                        // println!("{:?}", tag.get_formated_body());

                        // sleep_ms(100000000);
                        // description_line = Line::styled(format!("{} {}", tag.body, tag.body), COMPLETED_TEXT_FG_COLOR);
                    }
                }
            }
            _ => {
                text_lines.push(Line::styled("No mod selected.".to_string(), COMPLETED_TEXT_FG_COLOR));
                text = Text::from(text_lines);
                paragraph = Paragraph::new(text).gray().block(create_block("".to_string()));
            }
        }
        Clear.render(area, buffer);
        Widget::render(paragraph, area, buffer);
    }
}

// Goody two shoes struct
#[derive(Default)]
struct TabStorage {
    config_manager: Manager,
    // ordered_app_name_vector: Vec<String>,
    // ordered_app_vector: Vec<AppStruct>,
    list_state: ListState
}

impl TabStorage {
    fn get_app_names(&self) -> Vec<String>  {
        // if self.ordered_app_name_vector.len() < 1 {
        //     self.ordered_app_name_vector = self.config_manager.get_sorted_apps_name();
        // }
        // self.ordered_app_name_vector.to_owned()
        self.config_manager.get_app_names()
    }

    fn get_enabled_app_names(&self) -> Vec<String>  {
        self.config_manager.get_enabled_app_names()
        // if self.get_enabled_apps_name.len() < 1 {
        //     self.get_enabled_apps_name = self.config_manager.get_enabled_apps_name();
        // }
        // self.get_enabled_apps_name.to_owned()

    }
}



impl App {
    // pub(crate) fn run(mut self, terminal:  &mut DefaultTerminal) -> Result<()> {
    pub(crate) fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.reload_config();
        // let x =  terminal;
        // self.config_manager.load_config();
        // self.reset_active_tab_storage();
        // let mut widget_list_state = ListState::default();



        while self.running_state == AppState::Running {
            terminal.draw(|frame|
                frame.render_widget(&mut self, frame.area())
                // frame.render_stateful_widget(&mut self, frame.area(), &mut widget_list_state)
                // frame.render_stateful_widget(&self, frame.area(), &mut widged_state)
            )?;

            // let seconds = Duration::new(0, 200);
            // if event::poll(seconds)? {
            //     self.handle_events()?;
            // }
            // }
            self.handle_events()?;
        }
        Ok(())
    }

    // fn handle_events(&mut self) -> std::io::Result<()> {
    // fn handle_events(&mut self) -> std::io::Result<()> {
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
                            KeyCode::Enter => { self.toggle_enable_disable_mod()}
                            KeyCode::Up => { self.select_previous() }
                            KeyCode::Down => { self.select_next() }

                            // Tab Movement
                            KeyCode::Right => self.next_tab(),
                            KeyCode::Left => self.previous_tab(),

                            // Others
                            KeyCode::Char('q') | KeyCode::Char('Q')| KeyCode::Esc => self.quit(),
                            _ => {}
                        }
                    }
                    SelectedTab::Tab2 => {
                        match key.code {
                            // Tab specific
                            // KeyCode::Char('u') | KeyCode::Char('U')=> { self.download_patches() }
                            KeyCode::Char('s') | KeyCode::Char('S')=> { self.pull_latest_tags() } // Only find the latest for each app
                            // KeyCode::Char('u') | KeyCode::Char('U')=> { terminal.draw(update_app)?;     sleep_ms(10000000); } // Only find the latest for each app
                            // KeyCode::Char('u') | KeyCode::Char('U')=> { self.test_popup(terminal) } // Only find the latest for each app
                            KeyCode::Char('u') | KeyCode::Char('U')=> { self.update_all_enabled_mods() } // Only find the latest for each app
                            // KeyCode::Char('u') | KeyCode::Char('U')=> { self.update_all_enabled_mods(terminal) } // Only find the latest for each app
                            // KeyCode::Char('u') | KeyCode::Char('U')=> { terminal.draw(self.update_all_enabled_mods)?; sleep_ms(100000); } // Only find the latest for each app
                            // KeyCode::Char('u') | KeyCode::Char('U')=> { self.update_all_enabled_mods(terminal) } // Only find the latest for each app
                            // KeyCode::Char('p') | KeyCode::Char('P') => { self.patch() }

                            // Movement
                            // KeyCode::Enter => { self.toggle_enable_disable_mod()} // Update single one (or in the future open a new window to select the desired patch :shrug:)
                            KeyCode::Up => { self.select_previous() }
                            KeyCode::Down => { self.select_next() }

                            // Tab Movement
                            KeyCode::Right => self.next_tab(),
                            KeyCode::Left => self.previous_tab(),

                            // Others
                            KeyCode::Char('q') | KeyCode::Char('Q')| KeyCode::Esc => self.quit(),
                            _ => {}
                        }
                    }
                    // SelectedTab::Tab3 => {}
                    // SelectedTab::Tab4 => {}
                    _ => {
                        match key.code {
                            // Tab Movement
                            KeyCode::Right => self.next_tab(),
                            KeyCode::Left => self.previous_tab(),

                            // Others
                            KeyCode::Char('q') | KeyCode::Char('Q')| KeyCode::Esc => self.quit(),
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn select_next(&mut self) {
        self.active_tab_storage.list_state.select_next();
    }
    fn patch(&mut self) {
        self.active_tab_storage.list_state.select_next();
    }
    fn select_previous(&mut self) {
        self.active_tab_storage.list_state.select_previous();
    }

    fn select_first(&mut self) {
        self.active_tab_storage.list_state.select_first();
    }

    fn select_last(&mut self) {
        self.active_tab_storage.list_state.select_last();
    }

    // Tabs
    fn next_tab(&mut self) {
        let prev = self.selected_tab;
        self.selected_tab = self.selected_tab.next();
        if prev.to_string() != self.selected_tab.to_string() {
            self.reset_active_tab_storage();
            // self.active_tab_storage.list_state = ListState::default();
            // self.reload_config();
        }
    }

    fn previous_tab(&mut self) {
        let prev = self.selected_tab;
        self.selected_tab = self.selected_tab.previous();
        if prev.to_string() != self.selected_tab.to_string() {
            self.reset_active_tab_storage();
            // self.active_tab_storage.list_state = ListState::default();
            // self.config_manager.load_config();
        }
    }

    // State
    fn reset_active_tab_storage(&mut self) {
        self.active_tab_storage = TabStorage::default();
        self.reload_config();
        // self.reload_config();
        // self.active_tab_storage.config_manager = self.config_manager.clone();
    }

    fn quit(&mut self) {
        self.running_state = AppState::Quitting;
    }


    // fn download_mod(&mut self, app_name: String) {
    //
    // }

    // fn test_popup(&mut self, terminal: &mut DefaultTerminal) {
    //     // terminal.d
    //     // println!("Test popup!");
    //     // println!("Test popup!");
    //     // println!("Test popup!");
    //     // println!("Test popup!");
    //     // println!("Test popup!");
    //     // sleep_ms(10000000);
    // }

    fn update_all_enabled_mods(&mut self) {
        self.current_sub_menu=SubMenus::UpdateAllApps; // Select menu to render
    }

    fn pull_latest_tags(&mut self) {
        // let mut tags_hashmap:HashMap<String, TagInfo> = HashMap::new();
        for app_name in self.active_tab_storage.get_enabled_app_names() {
            let result = self.active_tab_storage.config_manager.config.apps.get(&app_name).unwrap().get_latest_tag();
            match result {
                Ok(new_tag) => {
                    self.latest_pulled_tags_hashmap.insert(app_name, new_tag);
                }
                Err(e) => {
                    // println!("Error getting tag for app '{}': << {} >>", app_struct.get_app_name(), e);
                    // exit(1);
                }
            }
        // self.latest_tags_pulled_map
        }
    }
    fn reload_config(&mut self) {
        // self.config_manager=Manager::default();
        // self.config_manager.load_config();
        // self.active_tab_storage = TabStorage::default();
        // self.reload_config();
        self.active_tab_storage.config_manager = Manager::default();
        self.active_tab_storage.config_manager.load_config();
    }

    fn save_config(&mut self) {
        self.active_tab_storage.config_manager.save_config();
    }


    // Tab 1
    fn toggle_enable_disable_mod(&mut self) {
        // println!("{:?}", self.tab_storage.list_state.selected());
        // sleep_ms(1000000);
        match self.active_tab_storage.list_state.selected() {
            Some(index) => {
                let app_list = self.active_tab_storage.get_app_names(); // self.config_manager.get_sorted_apps_name();
                let app = self.active_tab_storage.config_manager.config.apps.get_mut(app_list.get(index).unwrap()).unwrap();
                app.enabled ^= true;
                // let app = self.active_tab_storage.get_sorted_apps_name();
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
        // TODO IDK
        // let mut error_lines_hashmap = HashMap::new();

        // Get render areas
        use Constraint::{Length, Min};
        // let vertical = Layout::vertical([Length(1), Min(0), Min(0), Length(1)]);
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);
        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);


        match self.selected_tab {
            SelectedTab::Tab1 => self.selected_tab.render_enable_mods_tab(inner_area, buf, &mut self.active_tab_storage),
            SelectedTab::Tab2 => {
                let split_inner_area_vertical = Layout::horizontal([Min(0), Min(0)]);
                let [main_content_area, bottom_content_area] = split_inner_area_vertical.areas(inner_area);

                self.selected_tab.render_update_mods_tab(main_content_area, buf, &mut self.active_tab_storage, &mut self.latest_pulled_tags_hashmap);
                self.selected_tab.describe_selected_mod_tag_description(bottom_content_area, buf, &mut self.active_tab_storage, &mut self.latest_pulled_tags_hashmap);
            },
            _ => {
                //println!("tab out of bounds!")
            }
        }

        render_footer(self,footer_area,buf);
        // Popups render AFTER
        // Could be used to read errors -> then render the error popup.
        match self.current_sub_menu  {
            // Update "submenu"
            SubMenus::UpdateSingleApps|SubMenus::UpdateAllApps => {
                // Update

                // Reset hmap
                self.update_apps_status_hashmap = HashMap::new();

                // Get app list
                let mut app_name_list: Vec<String> = match self.current_sub_menu {
                    SubMenus::UpdateSingleApps => {
                        match self.active_tab_storage.list_state.selected() {
                            None => {vec![]} // TODO if no app is selected this shouldn't be rendered when UpdateSingleApp
                            Some(index) => {
                                vec![self.active_tab_storage.get_enabled_app_names().get(index).unwrap().to_string()]
                            }
                        }
                    }
                    _ => { self.active_tab_storage.get_enabled_app_names()}
                };
                // Format
                // Apps to update:
                // - 1 (waiting)
                // - 2 (waiting)
                // - 3 (waiting)
                // ...
                // - 1 (waiting)
                // - 2 (waiting)
                // - 3 (waiting)

                // idk, something about formatting the output
                // let mut lines_hashmap: HashMap<String,&String> = HashMap::new();

                for app_name in &app_name_list {
                    self.update_apps_status_hashmap.insert(
                        app_name.to_string(),
                        AppUpdatingStatus {
                            app_name: app_name.to_string(),
                            status: AppUpdatingStatusStatus::Pending
                        });
                }

                for app_name in app_name_list {
                    match self.update_apps_status_hashmap.get_mut(&app_name) {
                        None => {} // Pass
                        Some(app_update_status) => {
                            app_update_status.status = AppUpdatingStatusStatus::OnGoing;
                        }
                    }
                    // If no tag found custom message or something idk.
                    // crate::IntellijRustImportsMock::render_update_status(&update_apps_status_hashmap, popup_area, buf);

                    // Get latest tag
                    let latest_tag_info = self.latest_pulled_tags_hashmap.get(&app_name).unwrap();

                    // Attempt to download the latest tag
                    match self.active_tab_storage.config_manager.update_app(app_name.to_string(), latest_tag_info) {
                    // match true { // Testing
                        Ok(_) => {
                            // Ok(_) => {
                            match self.update_apps_status_hashmap.get_mut(&app_name) {
                                None => {} // Pass
                                Some(app_update_status) => {
                                    app_update_status.status = AppUpdatingStatusStatus::Updated;
                                }
                            }
                            self.save_config() // TODO reenable, testing
                        },
                        Err(e) => {
                            match self.update_apps_status_hashmap.get_mut(&app_name) {
                                None => {} // Pass
                                Some(app_update_status) => {
                                    app_update_status.status = AppUpdatingStatusStatus::Failed;
                                }
                            }
                        }
                        _ => {
                            // Err(e) => {
                            // Stop and print error
                        }
                    }
                    // Only if it works >:(, if error Render error and break the loop.
                }

                // // Get lines, line the lines, render
                // let text = Text::from("hello");
                // let bad_popup = Paragraph::new(text)
                //     .wrap(Wrap { trim: true })
                //     .style(Style::new().yellow())
                //     .block(
                //         Block::new()
                //             .title("Updating apps")
                //             .title_style(Style::new().white().bold())
                //             .borders(Borders::ALL)
                //             .border_style(Style::new().red()),
                //     );
                // let x = Widget::render(bad_popup, popup_area, buf);
                // x.blink();

                // self.current_sub_menu=SubMenus::None;
                // Mark completed
                self.current_sub_menu=SubMenus::UpdateAllCompleted;


                // Render menu
                fn render_update_status(update_apps_status_hashmap: &HashMap<String, AppUpdatingStatus>, popup_area: Rect, buf: &mut Buffer) {
                    Clear.render(popup_area, buf);
                    let mut lines_vector: Vec<Line> = vec![];
                    for (_, app) in update_apps_status_hashmap {
                        lines_vector.push(Line::styled(format!(" {}", app.get_status_string()), app.get_status_render_colour()));
                    }

                    let text = Text::from(lines_vector);

                    let bad_popup = Paragraph::new(text)
                        .wrap(Wrap { trim: true })
                        .style(Style::new().yellow())
                        .block(
                            Block::new()
                                .title("Updating apps")
                                .title_style(Style::new().white().bold())
                                .borders(Borders::ALL)
                                .border_style(Style::new().red()),
                        );
                    let x = Widget::render(bad_popup, popup_area, buf);
                    // sleep_ms(500);
                }

                // take up a third of the screen vertically and half horizontally
                let popup_area = Rect {
                    x: area.width / 4,
                    y: area.height / 3,
                    width: area.width / 2,
                    height: area.height / 3,
                };
                Clear.render(popup_area, buf);

                render_update_status(&self.update_apps_status_hashmap, popup_area, buf);
                // sleep_ms(100);
            }
            _ => {} // Pass
        }
    }
}


impl SelectedTab {

    fn render_enable_mods_tab(self, area: Rect, buffer: &mut Buffer, tab_storage: &mut TabStorage) {

        let mut c=0;
        let mut styled_lines: Vec<ListItem> = vec![];
        for app_name in tab_storage.get_app_names() {
            let color = alternate_colors(c);
            c+=1;

            let app= tab_storage.config_manager.config.apps.get(&app_name).unwrap();

            let line: Line = match app.enabled {
                true => Line::styled(format!(" ✓ {}", app.get_app_name()), COMPLETED_TEXT_FG_COLOR),
                false => Line::styled(format!(" ☐ {}", app.get_app_name()), TEXT_FG_COLOR)
            };

            styled_lines.push(ListItem::new(line).bg(color));
            // styled_line.push(ListItem::from(manager.config.apps.get(&app_name).unwrap()).bg(color));
        }

        let list = List::new(styled_lines)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buffer, &mut tab_storage.list_state);
    }

    fn render_update_mods_tab(self, area: Rect, buffer: &mut Buffer, tab_storage: &mut TabStorage, latest_tags_pulled_map: &mut HashMap<String,TagInfo>) {

        let mut c=0;
        let mut styled_lines: Vec<ListItem> = vec![];
        for app_name in tab_storage.get_enabled_app_names() {
            let color = alternate_colors(c);

            let app= tab_storage.config_manager.config.apps.get(&app_name).unwrap();
            if app.enabled {

                // Latest patch downloaded
                let line: Line = match latest_tags_pulled_map.get(&app.get_app_name()) {
                    None => {Line::styled(format!(" ? {}", app.get_app_name()), GREY_TEXT_FG_COLOR)}  // Need to fetch updates
                    Some(value) => {
                        // Differs with latest pulled
                        match app.tag_name == value.tag_name && !app.tag_name.is_empty() {
                            true => {Line::styled(format!(" ✓ {}", app.get_app_name()), COMPLETED_TEXT_FG_COLOR)} // Up to date
                            false => {Line::styled(format!(" ! {}", app.get_app_name()), YELLOW_TEXT_FG_COLOR)} // "New" version found
                        }
                    }
                };
                c+=1;

                styled_lines.push(ListItem::new(line).bg(color));
            }
        }

        let list = List::new(styled_lines)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buffer, &mut tab_storage.list_state);
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Ratatui Tabs Example".bold().render(area, buf);
}

fn render_footer(app: &App, area: Rect, buf: &mut Buffer) {
    match app.selected_tab {
        SelectedTab::Tab1 => {
            Line::raw("Use ← ↓ ↑ → to navigate | Enter to Select/Deselect | S/s to save | R/r to reload config | Q/q to quit")
            // Line::raw("Use ◄ ▲ ▼ ► to navigate | Enter to Select/Deselect | S/s to save | R/r to reload config | Q/q to quit")
            // Line::raw("Use ↓↑ to move | ◄ ► to change tab | Enter to Select/Deselect | S/s to save | R/r to reload config | Q/q to quit")
                .centered()
                .render(area, buf);
        }
        SelectedTab::Tab2 => {
            // | Enter to Update Selected
            Line::raw("Use ← ↓ ↑ → to navigate | s/S Search Updates | u/U to update All | R/r to reload config | Q/q to quit")
            // Line::raw("Use ◄ ▲ ▼ ► to navigate | s/S Search Updates | u/U to update All | R/r to reload config | Q/q to quit")
            // Line::raw("Use ↓↑ to move | ◄ ► to change tab | s/S Search Updates | u/U to update All | R/r to reload config | Q/q to quit")
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

// impl From<&AppStruct> for ListItem<'_> {
//     fn from(app: &AppStruct) -> Self {
//         let line = match app.enabled {
//             true => Line::styled(format!(" ✓ {}", app.get_app_name()), COMPLETED_TEXT_FG_COLOR),
//             false => Line::styled(format!(" ☐ {}", app.get_app_name()), TEXT_FG_COLOR)
//         };
//         ListItem::new(line)
//     }
// }
