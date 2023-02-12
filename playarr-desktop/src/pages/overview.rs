use egui::{style::Margin, Sense, Ui, Vec2};
use libmpv::Mpv;

use crate::{server::FetchResult, ui::App};

use super::{Page, Pages};

pub struct Overview;

impl Page for Overview {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(app: &mut App, ui: &mut Ui, _mpv: &Mpv) {
        let shows = match app.client.get_all_series() {
            FetchResult::Loading => {
                ui.label("Loading..");
                return;
            }
            FetchResult::Error(msg) => {
                ui.label(msg);
                return;
            }
            FetchResult::Ok(shows) => shows,
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::none()
                .inner_margin(Margin {
                    top: 20.0,
                    bottom: 20.0,
                    left: 35.0,
                    right: 35.0,
                })
                .show(ui, |ui| {
                    ui.heading("All shows");
                    ui.add_space(10.0);
                    let columns = if ui.available_width() < 1280.0 {
                        6.0
                    } else if ui.available_width() < 1536.0 {
                        8.0
                    } else {
                        10.0
                    };

                    let spacing = 20.0;
                    let width = ui.available_width();
                    egui::Grid::new("shows")
                        .num_columns(columns as usize)
                        .max_col_width((width - (spacing * (columns - 1.0))) / (columns))
                        .spacing(Vec2::splat(spacing))
                        .show(ui, |ui| {
                            for (index, show) in shows.iter().enumerate() {
                                let frame = egui::Frame::none().rounding(5.0).show(ui, |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        let poster_url = show
                                            .images
                                            .iter()
                                            .find(|image| image.cover_type == "poster");

                                        let aspect = 0.68;
                                        if let Some(image) = app.network_image_cache.fetch_image(
                                            poster_url.unwrap().remote_url.clone().unwrap(),
                                        ) {
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
                                    let current_season = show
                                        .seasons
                                        .iter()
                                        .find(|season| season.statistics.episode_file_count != 0);

                                    app.navigate(Pages::Show {
                                        id: show.id,
                                        season: if let Some(season) = current_season {
                                            season.season_number
                                        } else {
                                            show.seasons.first().unwrap().season_number
                                        },
                                    });
                                }

                                if frame.response.interact(Sense::hover()).hovered() {
                                    // change cursor icon egui::CursorIcon::PointingHand
                                    ui.ctx().output().cursor_icon = egui::CursorIcon::PointingHand;
                                }

                                if (index as f32 + 1.0) % columns == 0.0 {
                                    ui.end_row();
                                }
                            }
                        });
                });
        });
    }
}
