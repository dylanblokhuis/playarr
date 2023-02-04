use egui::Ui;
use libmpv::{FileState, Mpv};

use crate::ui::App;

pub struct Overview;

impl Overview {
    pub fn render(app: &mut App, ui: &mut Ui, mpv: &Mpv) {
        ui.heading("Playarr");

        ui.text_edit_singleline(&mut app.state.filepath);

        if ui.button("Watch").clicked() {
            app.state.timestamp_last_mouse_movement = std::time::Instant::now();
            app.properties.playback = true;
            mpv.playlist_load_files(&[(&app.state.filepath, FileState::AppendPlay, None)])
                .unwrap();
        }
    }
}
