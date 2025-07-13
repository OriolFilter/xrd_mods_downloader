mod ratatui_app;
mod apps;

use std::io;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{TableState, Widget};
use ratatui_app::*;
use apps::*;
use ratatui::style::palette::tailwind::{SLATE};
use ratatui::text::Line;

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
        app_list: default_apps,
        colors: TableColors::new()
    };
    loop {
        terminal.draw(|frame| mods_table.draw_mods_table(frame))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Up => mods_table.select_previous(),
                    KeyCode::Down => mods_table.select_next(),
                    KeyCode::Enter => {},
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