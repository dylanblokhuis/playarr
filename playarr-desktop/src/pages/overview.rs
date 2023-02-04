use egui::Ui;
use libmpv::{FileState, Mpv};

use crate::ui::App;

pub struct Overview;

impl Overview {
    pub fn render(app: &mut App, ui: &mut Ui, mpv: &Mpv) {
        ui.heading("Playarr");

        ui.text_edit_singleline(&mut app.filepath);

        if ui.button("Watch").clicked() {
            app.timestamp_last_mouse_movement = std::time::Instant::now();
            app.properties.playback = true;
            mpv.playlist_load_files(&[(&app.filepath, FileState::AppendPlay, None)])
                .unwrap();
        }
    }
}
