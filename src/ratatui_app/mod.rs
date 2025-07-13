use std::collections::HashMap;
use std::io::BufRead;
use std::option::Option;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::style::palette::tailwind;
use ratatui::style::palette::tailwind::Palette;
use ratatui::text::Text;
use ratatui::widgets::{Cell, HighlightSpacing, Paragraph, Row, Table, TableState, Widget};
use crate::apps::AppStruct;
use ratatui::style::palette::tailwind::{GREEN, SLATE, STONE};

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

pub(crate) struct ModsTable {
    // mod_list
    pub(crate) sort_ascend: bool,
    pub(crate) state: TableState,
    // color_index: i32,
    pub(crate) app_list: HashMap<String,AppStruct>,
    pub(crate) colors: TableColors
}

pub(crate) struct TableColors {
    pub(crate) buffer_bg: Color,
    pub(crate) header_bg: Color,
    pub(crate) header_fg: Color,
    pub(crate) row_fg: Color,
    pub(crate) selected_row_style_fg: Color,
    pub(crate) selected_column_style_fg: Color,
    pub(crate) selected_cell_style_fg: Color,
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
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

impl ModsTable {
    pub(crate) fn draw_mods_table(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(frame.area());

        self.render_table(frame, rects[0]);
        // self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect)  {

        let header_style =  Style::default();
        let selected_row_style = Style::default();
            // .add_modifier(Modifier::REVERSED)
            // .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default(); //.fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default();
            // .add_modifier(Modifier::REVERSED)
            // .fg(self.colors.selected_cell_style_fg);

        // let header = ["Name"]
        let header = ["Mod Name", "Enabled"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows = self.app_list.values().into_iter().enumerate().map(|(i, app)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = [app.get_app_name().to_string(),app.enabled.to_string()];
            item.into_iter().map(|content| Cell::from(Text::from(content))).collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color)).height(1)
        });

        let mut app_name_max_col_length:u16 = 0;
        let mut enabled_max_col_length:u16 = 4;

        for app in self.app_list.values() {
            if app_name_max_col_length < app.get_app_name().len() as u16 {
                app_name_max_col_length = app.get_app_name().len() as u16;
            }
        }


        // let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(app_name_max_col_length + 1),
                Constraint::Min(enabled_max_col_length + 1)
            ],
        )
            .header(header)
            .row_highlight_style(selected_row_style)
            .column_highlight_style(selected_col_style)
            .cell_highlight_style(selected_cell_style)
            // .highlight_symbol(Text::from(vec![
            //     "".into(),
            //     bar.into(),
            //     bar.into(),
            //     "".into(),
            // ])).
            .bg(self.colors.buffer_bg)
            .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
        // frame.render_widget(Paragraph::new("hiaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").bg(SLATE.c200).fg(SLATE.c800), area);
        // Paragraph
        // frame.render_stateful_widget(t, area, &mut self.state);
    }

}
