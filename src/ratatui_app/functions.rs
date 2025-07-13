use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget};

pub(crate) fn render_footer(frame: &mut Frame, area: Rect) {
    frame.render_widget(Line::raw("Use ↓ ↑ to navigate | Enter to Select/Deselect | s/S to invert sorting | Q/q to quit").centered(),area);
    // match app.selected_tab {
    //     SelectedTab::Tab1 => {
    //         Line::raw("Use ← ↓ ↑ → to navigate | Enter to Select/Deselect | S/s to save | R/r to reload config | Q/q to quit")
    //             // Line::raw("Use ◄ ▲ ▼ ► to navigate | Enter to Select/Deselect | S/s to save | R/r to reload config | Q/q to quit")
    //             // Line::raw("Use ↓↑ to move | ◄ ► to change tab | Enter to Select/Deselect | S/s to save | R/r to reload config | Q/q to quit")
    //             .centered()
    //             .render(area, buf);
    //     }
    //     SelectedTab::Tab2 => {
    //         // | Enter to Update Selected
    //         Line::raw("Use ← ↓ ↑ → to navigate | s/S Search Updates | u/U to update All | R/r to reload config | Q/q to quit")
    //             // Line::raw("Use ◄ ▲ ▼ ► to navigate | s/S Search Updates | u/U to update All | R/r to reload config | Q/q to quit")
    //             // Line::raw("Use ↓↑ to move | ◄ ► to change tab | s/S Search Updates | u/U to update All | R/r to reload config | Q/q to quit")
    //             .centered()
    //             .render(area, buf);
    //     }
    //     // SelectedTab::Tab2 => {}
    //     // SelectedTab::Tab3 => {}
    //     // SelectedTab::Tab4 => {}
    //     _ => {
    //         Line::raw("◄ ► to change tab | Press q to quit")
    //             .centered()
    //             .render(area, buf);
    //     }
    // }
}