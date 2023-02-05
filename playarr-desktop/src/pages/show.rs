use egui::{RichText, Sense, Ui, Vec2};
use libmpv::Mpv;

use crate::{
    ui::{App, Page},
    widgets::icons::{icon, CHEVRON_LEFT_ICON},
};

pub struct Show;

impl Show {
    pub fn render(app: &mut App, ui: &mut Ui, mpv: &Mpv, id: i64) {
        egui::Frame::none().inner_margin(15.0).show(ui, |ui| {
            let shows = app.client.get_all_series();
            let Some(shows) = shows else {
                ui.label("Loading..");
                return;
            };
            let show = shows.iter().find(|s| s.id == id).unwrap();
            if icon(ui, &CHEVRON_LEFT_ICON).clicked() {
                app.state.page = Page::Overview;
            }
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 25.0;
                let poster_url = show
                    .images
                    .iter()
                    .find(|image| image.cover_type == "poster");

                if let Some(image) = app
                    .network_image_cache
                    .fetch_image(poster_url.unwrap().remote_url.clone())
                {
                    image.show_max_size(ui, Vec2::new(200.0, 400.0));
                } else {
                    ui.allocate_exact_size(Vec2::new(200.0, 300.0), Sense::click());
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading(&show.title);
                        ui.label(&show.year.to_string());
                        ui.label(RichText::new(&show.overview).size(14.0));

                        show.seasons.iter().for_each(|season| {
                            ui.label(format!(
                                "Season {} - {} episodes - {} downloaded",
                                &season.season_number.to_string(),
                                &season.statistics.total_episode_count.to_string(),
                                &season.statistics.episode_file_count.to_string()
                            ));
                        });

                        let Some(episodes) = app.client.get_episodes(id) else {
                        ui.label("Loading..");
                        return;
                    };

                        egui::Grid::new("episodes")
                            .num_columns(4)
                            .max_col_width((ui.available_width() - 15.0) / 4.0)
                            .spacing(Vec2::splat(15.0))
                            .show(ui, |ui| {
                                for (index, episode) in episodes.iter().enumerate() {
                                    if episode.images.is_empty() {
                                        ui.label("No image found");
                                        continue;
                                    }

                                    if let Some(img) = app
                                        .network_image_cache
                                        .fetch_image(episode.images.first().unwrap().url.clone())
                                    {
                                        img.show_max_size(
                                            ui,
                                            Vec2::new(ui.available_width(), 300.0),
                                        );
                                    } else {
                                        ui.allocate_exact_size(
                                            Vec2::new(ui.available_width(), 300.0),
                                            Sense::click(),
                                        );
                                    }

                                    if (index + 1) % 4 == 0 {
                                        ui.end_row();
                                    }
                                }
                            });

                        ui.add_space(15.0);
                    });
                });
            });
        });
    }
}
