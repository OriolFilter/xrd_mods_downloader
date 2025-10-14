use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;
use std::option::Option;
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Constraint::{Length, Min};
use ratatui::prelude::Line;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::style::palette::tailwind;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Cell, HighlightSpacing, List, ListItem, ListState, Row, Table, TableState};
use crate::apps::AppStruct;
// use crate::ratatui_app::functions::render_footer;

#[derive(Clone)]
pub(crate) enum AppMenuOptionsList {
    Launch,
    Patch,
    Download,
    Update,
    Description,

    // HideOrShow
}

impl fmt::Display for AppMenuOptionsList {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            AppMenuOptionsList::Launch => {"Launch mod"}
            AppMenuOptionsList::Patch => {"Patch mod"}
            AppMenuOptionsList::Update => {"Search for updates"}
            AppMenuOptionsList::Description => {"Mod's description"}
            AppMenuOptionsList::Download => {"Download mod"}
        })?;
        Ok(())
    }
}

#[derive(Default)]
pub(crate) enum MenuToRender {
    #[default]
    AppList,
    AppMenuOptions // Shows the option menu for that app
}

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

pub(crate) struct ModListTable {
    // mod_list
    pub(crate) sort_ascend: bool,
    pub(crate) state: TableState,
    // color_index: i32,
    pub(crate) apps_hashmap: HashMap<String,AppStruct>,
    pub(crate) colors: TableColors
}

pub(crate) struct TableColors {
    pub(crate) buffer_bg: Color,
    pub(crate) header_bg: Color,
    pub(crate) header_fg: Color,
    pub(crate) row_fg: Color,
    pub(crate) selected_row_style_fg: Color,
    pub(crate) selected_column_style_fg: Color,
    pub(crate) normal_row_color: Color,
    pub(crate) alt_row_color: Color,
    pub(crate) footer_border_color: Color,
}

impl TableColors {
    pub fn new() -> Self {
        let color = tailwind::GREEN;
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

impl ModListTable {
    pub(crate) fn select_next(&mut self) {
        self.state.select_next();
    }
    pub(crate) fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub(crate) fn select_first(&mut self) {
        self.state.select_first();
    }

    pub(crate) fn select_last(&mut self) {
        self.state.select_last();
    }


    pub(crate) fn render(&mut self, frame: &mut Frame) {
        // let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let vertical = &Layout::vertical([Min(0), Length(1)]);
        // let vertical = &Layout::vertical([Length(1), Min(0), Length(1)]);
        let rects = vertical.split(frame.area());
        self.render_table(frame, rects[0]);
        // Line::raw("Use ↓ ↑ to navigate | Enter to Select/Deselect | Q/q to quit").centered().render(rects[1]);
        // self.render_controls(frame, rects[1]);


        self.render_footer(frame,rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect)  {

        let app_list = self.get_visible_app_list();
        let block = Block::new()
            .borders(Borders::ALL)
            .title("Apps");

        let header_style =  Style::default();
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        // let selected_col_style = Style::default(); //.fg(self.colors.selected_column_style_fg);

        // let header = ["Name"]
        let header = ["Name", "Installed"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        // let rows = self.apps_hashmap.values().into_iter().enumerate().map(|(i, app)| {
        let rows = app_list.into_iter().enumerate().map(|(i, app_name)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = [app_name.to_string(), self.apps_hashmap.get(&app_name).unwrap().is_installed().to_string()];
            item.into_iter().map(|content| Cell::from(Text::from(content))).collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color)).height(1)
        });

        let mut app_name_max_col_length:u16 = 0;
        let mut enabled_max_col_length:u16 = 4;

        for app in self.apps_hashmap.values() {
            if app_name_max_col_length < app.get_app_name().len() as u16 {
                app_name_max_col_length = app.get_app_name().len() as u16;
            }
        }

        let bar = " > ";
        let table = Table::new(
                rows,
                [
                    // + 1 is for padding.
                    Constraint::Length(app_name_max_col_length + 1),
                    Constraint::Min(enabled_max_col_length + 1)
                ],
            )
            .header(header)
            .row_highlight_style(selected_row_style)
            // .column_highlight_style(selected_col_style)
            .highlight_symbol(Text::from(vec![
                bar.into()
            ]))
            .bg(self.colors.buffer_bg)
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);
        frame.render_stateful_widget(table, area, &mut self.state);
        // frame.render_widget(Paragraph::new("hiaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").bg(SLATE.c200).fg(SLATE.c800), area);
        // Paragraph
        // frame.render_stateful_widget(t, area, &mut self.state);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Line::raw("Use ↓ ↑ to navigate | Enter to Select/Deselect | r/R to revert sorting | Q/q to quit").centered(),area);
    }

    pub(crate) fn get_visible_app_list(&self) -> Vec<String> {
        let mut apps_name_list: Vec<String> = vec![];
        for app in self.apps_hashmap.values() {
            if !app.hidden {
                apps_name_list.push(app.get_app_name());
            }
        }
        apps_name_list.sort();
        if self.sort_ascend {
            apps_name_list
        } else {
            apps_name_list.reverse();
            apps_name_list
        }
    }
    pub(crate) fn get_mut_selected_app(&mut self) -> Option<&mut AppStruct> {
        match self.state.selected() {
            Some(index) => {
                let app_list = self.get_visible_app_list();
                let app = self.apps_hashmap.get_mut(&app_list.get(index).unwrap().to_string());
                app
            }
            _ => {Option::None}
        }
    }    pub(crate) fn get_selected_app(&self) -> Option<&AppStruct> {
        match self.state.selected() {
            Some(index) => {
                let app_list = self.get_visible_app_list();
                let app = self.apps_hashmap.get(&app_list.get(index).unwrap().to_string());
                app
            }
            _ => {Option::None}
        }
    }
        // pub (crate) fn get_app_names(&self) -> Vec<String> {
        //     // self.config.apps.keys().sorted().collect()
        //     let mut apps_name_list: Vec<String> = self.config.apps.iter().map(|(app_name,app)| {app.get_app_name()}).collect();
        //     apps_name_list.sort();
        //     apps_name_list
        // }
}


pub(crate) struct AppMenuOptions {
    pub(crate) state: ListState,
    pub(crate) app: Option<AppStruct>,
    pub(crate) colors: TableColors
}

impl AppMenuOptions {
    pub(crate) fn select_next(&mut self) {
        self.state.select_next();
    }
    pub(crate) fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub(crate) fn select_first(&mut self) {
        self.state.select_first();
    }

    pub(crate) fn select_last(&mut self) {
        self.state.select_last();
    }
    pub(crate) fn render(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let vertical = &Layout::vertical([Min(0), Length(1)]);
        // let vertical = &Layout::vertical([Length(1), Min(0), Length(1)]);
        let rects = vertical.split(frame.area());
        // self.render_table);
        // Line::raw("Use ↓ ↑ to navigate | Enter to Select/Deselect | Q/q to quit").centered().render(rects[1]);
        // self.render_controls(frame, rects[1]);
        // self.app.unwrap().get_app_name();
        // let _ = self.app.unwrap();

        self.render_menu(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Line::raw("Use ↓ ↑ to navigate | Enter to Select/Deselect | s/S to invert sorting | Q/q to quit").centered(),area);
    }

    fn get_menu_options_from_app(&self) -> Vec<AppMenuOptionsList>{
        let mut menu_options = vec![];

        match self.app.clone().unwrap().is_installed() {
            true => {
                if self.app.clone().unwrap().is_launchable() {
                    menu_options.push(AppMenuOptionsList::Launch)
                }

                menu_options.push(AppMenuOptionsList::Update);

                if self.app.clone().unwrap().is_patcheable(){
                    menu_options.push(AppMenuOptionsList::Patch)
                }
            }
            false => {
                menu_options.push(AppMenuOptionsList::Download)
            }
        }


        menu_options.push(AppMenuOptionsList::Description);

        menu_options
    }
    fn render_menu(&mut self, frame: &mut Frame, area: Rect)  {
        let app_name = self.app.clone().unwrap().get_app_name();
        let block = Block::new()
            .borders(Borders::ALL)
            .title(format!("Selected app {}",app_name));

        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);


        let mut menu_options = self.get_menu_options_from_app();


        let mut i=0;
        let mut styled_lines: Vec<ListItem> = vec![];
        for option in menu_options {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };

            // let line: Line = Line::styled(option);

            styled_lines.push(ListItem::new(option.to_string()).bg(color));
            // styled_line.push(ListItem::from(manager.config.apps.get(&app_name).unwrap()).bg(color));
            i+=1;
        }

        let list = List::new(styled_lines)
            .highlight_symbol(" > ")
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(selected_row_style)
            .bg(self.colors.buffer_bg)
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        frame.render_stateful_widget(list, area, &mut self.state);
    }

    pub(crate) fn get_selected_menu(&mut self) -> Option<AppMenuOptionsList> {
        match self.state.selected() {
            Some(index) => {
                let menu_options = self.get_menu_options_from_app();
                Some(menu_options.get(index).unwrap().clone())
            }
            _ => {Option::None}
        }
    }
}

