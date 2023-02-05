use egui::{Color32, Sense, Ui, Vec2};
use libmpv::Mpv;

use crate::ui::{App, Page};

pub struct Overview;

impl Overview {
    pub fn render(app: &mut App, ui: &mut Ui, mpv: &Mpv) {
        let shows = app.client.get_all_series();

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::none().inner_margin(15.0).show(ui, |ui| {
                ui.heading("Shows");
                egui::Grid::new("shows")
                    .num_columns(3)
                    .max_col_width(ui.available_width() / 3.0)
                    .spacing(Vec2::splat(15.0))
                    .show(ui, |ui| {
                        if let Some(shows) = shows {
                            for (index, show) in shows.iter().enumerate() {
                                let frame = egui::Frame::none()
                                    .fill(Color32::from_rgb(30, 41, 59))
                                    .inner_margin(5.0)
                                    .rounding(5.0)
                                    .show(ui, |ui| {
                                        ui.vertical_centered_justified(|ui| {
                                            ui.label(&show.title);
                                            ui.label(&show.year.to_string());

                                            let poster_url = show
                                                .images
                                                .iter()
                                                .find(|image| image.cover_type == "poster");

                                            if let Some(image) = app
                                                .network_image_cache
                                                .fetch_image(poster_url.unwrap().remote_url.clone())
                                            {
                                                image.show_max_size(ui, Vec2::new(100.0, 200.0));
                                            } else {
                                                ui.add_space(100.0);
                                            }
                                        });
                                    });

                                if frame.response.interact(Sense::click()).clicked() {
                                    app.state.page = Page::Show(show.id);
                                }

                                if frame.response.interact(Sense::hover()).hovered() {
                                    // change cursor icon egui::CursorIcon::PointingHand
                                    ui.ctx().output().cursor_icon = egui::CursorIcon::PointingHand;
                                }

                                if (index + 1) % 3 == 0 {
                                    ui.end_row();
                                }
                            }
                        } else {
                            ui.label("Loading...");
                        }
                    });
            });
        });

        // ui.text_edit_singleline(&mut app.state.filepath);

        // if ui.button("Watch").clicked() {
        //     app.state.timestamp_last_mouse_movement = std::time::Instant::now();
        //     app.properties.playback = true;
        //     mpv.playlist_load_files(&[(&app.state.filepath, FileState::AppendPlay, None)])
        //         .unwrap();
        // }
    }
}
