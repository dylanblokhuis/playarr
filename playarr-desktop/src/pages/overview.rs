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
                let columns = 5.0;
                let spacing = 15.0;
                let width = ui.available_width();
                egui::Grid::new("shows")
                    .num_columns(columns as usize)
                    .max_col_width((width - (spacing * columns)) / (columns))
                    .spacing(Vec2::splat(15.0))
                    .show(ui, |ui| {
                        if let Some(shows) = shows {
                            for (index, show) in shows.iter().enumerate() {
                                let frame = egui::Frame::none().rounding(5.0).show(ui, |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        let poster_url = show
                                            .images
                                            .iter()
                                            .find(|image| image.cover_type == "poster");

                                        let aspect = 0.68;
                                        if let Some(image) = app
                                            .network_image_cache
                                            .fetch_image(poster_url.unwrap().remote_url.clone())
                                        {
                                            let desired_width = ui.available_width();
                                            let desired_height = desired_width / aspect;
                                            image.show_size(
                                                ui,
                                                Vec2::new(desired_width, desired_height),
                                            );
                                        } else {
                                            ui.allocate_space(Vec2::new(
                                                ui.available_width(),
                                                ui.available_width() / aspect,
                                            ));
                                        }
                                        // ui.label(&show.title);
                                        // ui.label(&show.year.to_string());
                                    });
                                });

                                if frame.response.interact(Sense::click()).clicked() {
                                    app.state.page = Page::Show(show.id);
                                }

                                if frame.response.interact(Sense::hover()).hovered() {
                                    // change cursor icon egui::CursorIcon::PointingHand
                                    ui.ctx().output().cursor_icon = egui::CursorIcon::PointingHand;
                                }

                                if (index as f32 + 1.0) % columns == 0.0 {
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
