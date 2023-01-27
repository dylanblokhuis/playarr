use std::sync::{Arc, Mutex};

use egui::Ui;
use libmpv::{Mpv, FileState};

pub struct App {
    mpv: Arc<Mutex<Mpv>>,
    filepath: String,

}

impl App {
    pub fn new(mpv:Arc<Mutex<Mpv>>) -> Self {
        Self { mpv: mpv, filepath: String::new() }
    }

    pub fn render(&mut self, ui: &mut Ui) -> egui::InnerResponse<()> {
        egui::Frame::none()
            .fill(egui::Color32::BLACK)
            .inner_margin(0.0)
            .outer_margin(0.0)
            .show(ui, |ui| {
                ui.heading("Playarr");

                ui.text_edit_singleline(&mut self.filepath);

                if ui.button("Play").clicked() {
                  self.mpv.lock().unwrap().playlist_load_files(&[(&self.filepath, FileState::AppendPlay, None)])
                  .unwrap();
                }
                

                if ui.button("Pause").clicked() {
                    self.mpv.lock().unwrap().pause().unwrap();
                }

                if ui.button("Stop").clicked() {
                    self.mpv.lock().unwrap().playlist_remove_current().unwrap();
                }
            })
    }
}
