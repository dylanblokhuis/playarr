use egui::{style::Margin, Frame, Sense, Ui, Vec2};

use libmpv::Mpv;

use crate::{
    server::{serde::Show, FetchResult},
    ui::App,
};

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
                    /*
                     * Recently added
                     */
                    {
                        ui.heading("Recently aired");
                        let mut recently_aired = shows.clone();
                        recently_aired.sort_by(|a, b| b.previous_airing.cmp(&a.previous_airing));
                        grid(
                            "recently_aired",
                            recently_aired,
                            GridOptions {
                                show_time_since_last_airing: true,
                                max_rows: Some(1),
                            },
                            ui,
                            app,
                        );
                    }
                    ui.add_space(10.0);

                    ui.heading("All shows");
                    grid(
                        "shows",
                        shows,
                        GridOptions {
                            show_time_since_last_airing: false,
                            max_rows: None,
                        },
                        ui,
                        app,
                    );
                });
        });
    }
}

struct GridOptions {
    show_time_since_last_airing: bool,
    max_rows: Option<i64>,
}
fn grid(id: &str, shows: Vec<Show>, options: GridOptions, ui: &mut Ui, app: &mut App) {
    let columns = if ui.available_width() < 1280.0 {
        6.0
    } else if ui.available_width() < 1536.0 {
        8.0
    } else {
        10.0
    };

    let spacing = 20.0;
    let width = ui.available_width();
    egui::Grid::new(id)
        .num_columns(columns as usize)
        .max_col_width((width - (spacing * (columns - 1.0))) / (columns))
        .spacing(Vec2::splat(spacing))
        .show(ui, |ui| {
            let shows = if let Some(max_rows) = options.max_rows {
                shows[0..(max_rows as f32 * columns) as usize].to_vec()
            } else {
                shows
            };

            for (index, show) in shows.iter().enumerate() {
                let frame = Frame::none().rounding(5.0).show(ui, |ui| {
                    ui.vertical(|ui| {
                        let poster_url = show
                            .images
                            .iter()
                            .find(|image| image.cover_type == "poster");

                        let aspect = 0.68;
                        if let Some(image) = app
                            .network_image_cache
                            .fetch_image(poster_url.unwrap().remote_url.clone().unwrap())
                        {
                            let desired_width = ui.available_width();
                            let desired_height = desired_width / aspect;
                            image.show_size(ui, Vec2::new(desired_width, desired_height));
                        } else {
                            ui.allocate_space(Vec2::new(
                                ui.available_width(),
                                ui.available_width() / aspect,
                            ));
                        }

                        if show.previous_airing.is_some() && options.show_time_since_last_airing {
                            ui.spacing_mut().item_spacing.y = 5.0;
                            Frame::none()
                                .inner_margin(Margin {
                                    left: 5.0,
                                    right: 5.0,
                                    top: 0.0,
                                    bottom: 0.0,
                                })
                                .show(ui, |ui| {
                                    if let Some(current_season) = show.seasons.last() {
                                        ui.label(format!(
                                            "S{} - E{}",
                                            current_season.season_number,
                                            current_season.statistics.episode_count
                                        ));
                                    }

                                    let prev_airing = chrono::DateTime::parse_from_rfc3339(
                                        &show.previous_airing.clone().unwrap(),
                                    )
                                    .unwrap();
                                    let diff =
                                        chrono::Utc::now().signed_duration_since(prev_airing);
                                    ui.label(if diff.num_days() > 0 {
                                        format!("{} days ago", diff.num_days())
                                    } else if diff.num_minutes() > 60 {
                                        format!("{} hours ago", diff.num_hours())
                                    } else {
                                        format!("{} minutes ago", diff.num_minutes())
                                    });
                                });
                        }
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
}
