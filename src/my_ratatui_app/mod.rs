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

#[derive(Default)]
struct AppStructListMenu {
    apps: Vec<AppStruct>,
    state: ListState
}

#[derive(Debug)]
struct TodoList {
    items: Vec<TodoItem>,
    state: ListState,
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

impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, todo, info)| TodoItem::new(status, todo, info))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl TodoItem {
    fn new(status: Status, todo: &str, info: &str) -> Self {
        Self {
            status,
            todo: todo.to_string(),
            info: info.to_string(),
        }
    }
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
    app_struct_list_menu: AppStructListMenu
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

#[derive(Default)]
struct ItemsList {
    config_manager:  Manager,
    index: i32
}

impl App {
    pub(crate) fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {

        let mut config_manager = Manager::default();
        // config_manager: Manager
        config_manager.load_config();

        // let mut app_names_sorted: Vec<String> = config_manager.config.apps.iter().map(|(app_name,app)| {app_name.to_string()}).collect(); // Get app_names_sorted
        let mut app_names_sorted: Vec<String> = vec![];
        for name in config_manager.config.apps.keys() {
            app_names_sorted.push(name.to_string());
        }

        let mut app_sorted: Vec<AppStruct> = vec![    ];
        app_names_sorted.sort();

        for app_name in app_names_sorted {
            // app_sorted.push(*config_manager.config.apps.get(&app_name).unwrap());
            app_sorted.push(config_manager.config.apps.get(&app_name).unwrap().clone());
        }

        // sleep_ms(10000000);
        self.app_struct_list_menu.apps=app_sorted;

        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
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
                            KeyCode::Right => self.next_tab(),
                            KeyCode::Left => self.previous_tab(),
                            KeyCode::Char('q') | KeyCode::Char('Q')| KeyCode::Esc => self.quit(),
                            KeyCode::Char('s') => {},
                            _ => {}
                        }
                    }
                    // SelectedTab::Tab2 => {}
                    // SelectedTab::Tab3 => {}
                    // SelectedTab::Tab4 => {}
                    _ => {
                        match key.code {
                            KeyCode::Right => self.next_tab(),
                            KeyCode::Left => self.previous_tab(),
                            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => self.quit(),
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let mut config_manager = Manager::default();
        // config_manager: Manager
        config_manager.load_config();

        // let mut app_names_sorted: Vec<String> = config_manager.config.apps.iter().map(|(app_name,app)| {app_name.to_string()}).collect(); // Get app_names_sorted
        let mut app_names_sorted: Vec<String> = vec![];
        for name in config_manager.config.apps.keys() {
            app_names_sorted.push(name.to_string());
        }

        let mut app_sorted: Vec<AppStruct> = vec![    ];
        app_names_sorted.sort();

        for app_name in app_names_sorted {
            // app_sorted.push(*config_manager.config.apps.get(&app_name).unwrap());
            app_sorted.push(config_manager.config.apps.get(&app_name).unwrap().clone());
        }




        ///
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        // self.selected_tab.render(inner_area, buf);

        match self.selected_tab {
            SelectedTab::Tab1 => self.selected_tab.render_tab0(inner_area, buf, app_sorted),
            SelectedTab::Tab2 => self.selected_tab.render_tab1(inner_area, buf),
            SelectedTab::Tab3 => self.selected_tab.render_tab2(inner_area, buf),
            SelectedTab::Tab4 => self.selected_tab.render_tab3(inner_area, buf),
        }

        render_footer(self,footer_area, buf);
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

fn render_title(area: Rect, buf: &mut Buffer) {
    "Ratatui Tabs Example".bold().render(area, buf);
}

fn render_footer(app: &App, area: Rect, buf: &mut Buffer) {
    match app.selected_tab {
        SelectedTab::Tab1 => {
            Line::raw("◄ ► to change tab | Press s to save | Press q to quit")
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

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_tab0(self, area: Rect, buf: &mut Buffer, app_list: Vec<AppStruct>) {

        // let mut items_list = vec![];
        // for app in app_sorted {
        //     let color = alternate_colors(c);
        //     items_list.push(ListItem::from(app).bg(color));
        //     c+=1;
        // }

        // let mut todo_list = TodoList::from_iter([
        //         (Status::Todo, "Rewrite everything with Rust!", "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust"),
        //         (Status::Completed, "Rewrite all of your tui apps with Ratatui", "Yes, you heard that right. Go and replace your tui with Ratatui."),
        // ]);

        // let items_list: Vec<ListItem> = todo_list
        //     .items
        //     .iter()
        //     .enumerate()
        //     .map(|(i, todo_item)| {
        //         let color = alternate_colors(i);
        //         ListItem::from(todo_item).bg(color)
        //     })
        //     .collect();

        // let items: Vec<ListItem> = config_manager.config.apps.iter().map(|(app_name,app) | { let color = alternate_colors(c); c +=1 ; ListItem::new(app).bg(color) } ).collect();
        let mut c=0;
        // // let items_list: Vec<ListItem> = app_list.iter().map(| app | { let color = alternate_colors(c); c +=1 ; ListItem::new(app).bg(color) } ).collect();
        // let items_list: Vec<ListItem> = app_list.iter().map(| app | { let color = alternate_colors(c); c +=1 ; ListItem::new(app.get_app_name()).bg(color) } ).collect();
        // let items_list: Vec<ListItem> = app_list.iter().map(| app | { let color = alternate_colors(c); c +=1 ; ListItem::new(app).bg(color) } ).collect();

        // let list = List::new(items_list)
        //     .block(self.block())
        //     // .block(Block::bordered().title("List"))
        //     .highlight_style(Style::new().reversed())
        //     .highlight_symbol(">>")
        //     .repeat_highlight_symbol(true);


        let mut items_list: Vec<AppStruct> = vec![];
        // let mut new_items_list: Vec<ListItem> = vec![];

        for app in app_list {

            // let x = TodoItem{
            //     todo: app.repo_owner.to_string(),
            //     info: app.repo_owner.to_string(),
            //     status: Status::Todo,
            // };
            // println!("{:#?}",app.repo_owner);
            items_list.push(app);
        }



        // for app in items_list {
        //     let z = ListItem::new(&app);
        //     // new_items_list.push(z);
        // }

        let new_items_list: Vec<ListItem> = items_list
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        let list = List::new(new_items_list)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);


        // StatefulWidget::render(list, area, buf, &mut items_list.state);
        Widget::render(list, area, buf);
        // let new_items_list: Vec<ListItem> = items_list.iter()
        //     .map(|todo_item| {
        //         ListItem::from(todo_item)
        //     })
        //     .collect();


        // for x in items_list {
        //     let z = ListItem::from(x);
        // }

        // println!("fuck");
        // returnlist;
        // frame.render_stateful_widget(list, area, &mut state);


        // items.render(area, buf);
        // list.render(area, buf);
        // Paragraph::new("Tab1")
        //     .block(self.block())
        //     .render(area, buf);

        // println!("{:#?}",todo_list);
        // println!("{:#?}",items_list);

        // use thread::sleep_ms;
        // //
        // println!("fuck");
        // sleep_ms(100000000);
        // Paragraph::new("test")
        //     .block(self.block())
        //     .fg(TEXT_FG_COLOR)
        //     .wrap(Wrap { trim: false })
        //     .render(area, buf);
        // StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
        // Paragraph::new("Look! I'm different than others!")
        //     .block(self.block())
        //     .render(area, buf);
        // let list = List::new(new_items_list)
        //     .highlight_symbol(">")
        //     .highlight_spacing(HighlightSpacing::Always);

        //
        // // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // // same method name `render`.
        // // StatefulWidget::render(list, area, buf, );
        //
        // Paragraph::new("Tab1")
        //     .block(self.block())
        //     .render(area, buf);



    }

    fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome to the Ratatui tabs example!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab2(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab3(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("I know, these are some basic changes. But I think you got the main idea.")
            .block(self.block())
            .render(area, buf);
    }

    /// A block surrounding the tab's content
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

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!(" ☐ {}", value.todo), TEXT_FG_COLOR),
            Status::Completed => {
                Line::styled(format!(" ✓ {}", value.todo), COMPLETED_TEXT_FG_COLOR)
            }
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