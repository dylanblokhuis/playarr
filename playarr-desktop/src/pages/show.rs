use egui::{style::Margin, RichText, Sense, Ui, Vec2};
use libmpv::Mpv;

use crate::{server::FetchResult, ui::App, utils::season_or_specials_label, widgets::breadcrumbs};

use super::{Page, Pages};

#[derive(Clone)]
pub struct Show {
    pub id: i64,
    pub season: i64,
}

impl Page for Show {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(app: &mut App, ui: &mut Ui, mpv: &Mpv) {
        let id = app.get_page_state::<Self>().id;
        let season_nr = app.get_page_state::<Self>().season;

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

        let mut show = shows.iter().find(|s| s.id == id).unwrap().to_owned();

        if breadcrumbs(
            ui,
            vec![
                "Overview".into(),
                format!(
                    "{} ({})",
                    show.title.clone(),
                    season_or_specials_label(season_nr)
                ),
            ],
        )
        .clicked()
        {
            app.navigate(Pages::Overview);
        }

        egui::Frame::none()
            .inner_margin(Margin {
                top: 0.0,
                bottom: 20.0,
                left: 35.0,
                right: 0.0,
            })
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 35.0;
                    let poster_url = show
                        .images
                        .iter()
                        .find(|image| image.cover_type == "poster");

                    if let Some(image) = app
                        .network_image_cache
                        .fetch_image(poster_url.unwrap().remote_url.clone().unwrap())
                    {
                        image.show_max_size(ui, Vec2::new(200.0, 400.0));
                    } else {
                        ui.allocate_exact_size(Vec2::new(200.0, 300.0), Sense::click());
                    }

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Frame::none()
                            .inner_margin(Margin {
                                top: 0.0,
                                bottom: 0.0,
                                left: 0.0,
                                right: 35.0,
                            })
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.heading(&show.title);
                                    ui.label(&show.year.to_string());
                                    ui.label(RichText::new(&show.overview).size(16.0));

                                    ui.horizontal(|ui| {
                                        show.seasons.sort_by(|a, b| {
                                            if a.season_number == 0 {
                                                return std::cmp::Ordering::Greater;
                                            }

                                            if b.season_number == 0 {
                                                return std::cmp::Ordering::Less;
                                            }

                                            a.season_number.cmp(&b.season_number)
                                        });

                                        show.seasons
                                            .iter()
                                            .filter(|season| season.statistics.episode_count > 0)
                                            .for_each(|season| {
                                                if ui
                                                    .selectable_label(
                                                        season.season_number == season_nr,
                                                        season_or_specials_label(
                                                            season.season_number,
                                                        ),
                                                    )
                                                    .clicked()
                                                {
                                                    app.navigate(Pages::Show {
                                                        id,
                                                        season: season.season_number,
                                                    })
                                                }
                                            });
                                    });

                                    let episodes = match app.client.get_episodes(id) {
                                        FetchResult::Loading => {
                                            ui.label("Loading..");
                                            return;
                                        }
                                        FetchResult::Error(msg) => {
                                            ui.label(msg);
                                            return;
                                        }
                                        FetchResult::Ok(episodes) => episodes,
                                    };

                                    egui::Grid::new("episodes")
                                        .num_columns(4)
                                        .max_col_width((ui.available_width() - 45.0) / 4.0)
                                        .spacing(Vec2::splat(15.0))
                                        .show(ui, |ui| {
                                            for (index, episode) in episodes
                                                .iter()
                                                .filter(|episode| {
                                                    episode.season_number == season_nr
                                                })
                                                .enumerate()
                                            {
                                                let wrapper = egui::Frame::none()
                                                    .show(ui, |ui| {
                                                        ui.vertical(|ui| {
                                                            if episode.images.is_empty() {
                                                                ui.label("No image found");

                                                                return;
                                                            }

                                                            if let Some(img) =
                                                                app.network_image_cache.fetch_image(
                                                                    episode
                                                                        .images
                                                                        .first()
                                                                        .unwrap()
                                                                        .url
                                                                        .clone(),
                                                                )
                                                            {
                                                                img.show_max_size(
                                                                    ui,
                                                                    Vec2::new(
                                                                        ui.available_width(),
                                                                        300.0,
                                                                    ),
                                                                );
                                                            } else {
                                                                ui.allocate_exact_size(
                                                                    Vec2::new(
                                                                        ui.available_width(),
                                                                        300.0,
                                                                    ),
                                                                    Sense::click(),
                                                                );
                                                            }
                                                            ui.label(&episode.title);
                                                            ui.label(format!(
                                                                "Episode {}",
                                                                episode.episode_number
                                                            ));
                                                        });
                                                    })
                                                    .response
                                                    .interact(Sense::click());

                                                if wrapper.clicked() && episode.has_file {
                                                    app.navigate(Pages::Episode { id: episode.id })
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
            });
    }
}
