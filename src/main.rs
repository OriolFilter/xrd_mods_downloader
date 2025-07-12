mod ratatui_app;
mod apps;

use std::io;
use ratatui_app::*;

fn main() -> io::Result<()>  {
    println!("hi");

    color_eyre::install();
    let terminal = ratatui::init();

    let app_result = crate::ratatui_app::App::new(terminal).run();
    // let app_result = crate::actual_new_ratatui_app::App::default().run(terminal);
    // let app_result = crate::actual_new_ratatui_app::App::default().run();
    ratatui::restore();
    println!("bye");
    Ok(())
    // app_result
}