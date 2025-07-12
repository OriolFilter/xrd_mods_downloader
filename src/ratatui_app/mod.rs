use std::collections::HashMap;
use std::option::Option;
use ratatui::DefaultTerminal;
use ratatui::widgets::TableState;
use crate::apps::AppStruct;

#[derive(Default)]
pub(crate) struct App {
    // mod_list: Option<Vec<String>>
    ratatui_terminal: Option<DefaultTerminal>,
    mods_table: Option<ModsTable>
}



struct ModsTable {
    // mod_list
    sort_ascend: bool,
    state: TableState,
    color_index: i32,
    app_list: HashMap<String,AppStruct>,
}


impl App {
    pub(crate) fn new(terminal: DefaultTerminal) -> App {
        Self {
            ratatui_terminal: Some(terminal),
            mods_table: None,
        }
    }
}

impl App {
    fn print_mods_table() {
        // Call print
        
    }
    
    fn generate_mods_table(&mut self) {
        // Get mods (from config TODO)
        let default_apps = crate::apps::get_default_apps();

        // Store mods in struct
        self.mods_table = Option::from(ModsTable {
            sort_ascend: false,
            state: TableState::default().with_selected(0),
            color_index: 0,
            app_list: default_apps,
        })
    }
    
}


impl App {
    pub fn run(mut self) {
        println!("hi fuck hi");
        
        //
        // Find config

        // If no config found
            // Ask to generate a new one
                // yes -> continue
                // false -> quit

        // Ok(())
    }
}