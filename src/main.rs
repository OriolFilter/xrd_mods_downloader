mod ratatui_app;
mod apps;

use std::io;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::widgets::TableState;
use ratatui_app::*;
use apps::*;

fn main() -> io::Result<()>  {
    println!("hi");

    // Check if config exists
    let default_apps = crate::apps::get_default_apps();

    // If it doesn't ask whether create a new one or not
    // Either leave or create a new one.

    // If it does continue
    color_eyre::install();
    let mut terminal = ratatui::init();

    let mut mods_table = ModsTable {
        sort_ascend: false,
        state: TableState::default().with_selected(0),
        // color_index: 0,
        app_list: default_apps,
        // colors: TableColors{
        //     buffer_bg: Default::default(),
        //     header_bg: Default::default(),
        //     header_fg: Default::default(),
        //     row_fg: Default::default(),
        //     selected_row_style_fg: Default::default(),
        //     selected_column_style_fg: Default::default(),
        //     selected_cell_style_fg: Default::default(),
        //     normal_row_color: Default::default(),
        //     alt_row_color: Default::default(),
        //     footer_border_color: Default::default(),
        // }
    };
    loop {
        terminal.draw(|frame| mods_table.draw_mods_table(frame))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }


    std::thread::sleep_ms(10000);






    ratatui::restore();
    println!("bye");
    Ok(())
    // app_result
}