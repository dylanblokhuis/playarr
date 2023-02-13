use egui::{style::Margin, RichText, Sense, Ui, Vec2};
use libmpv::{FileState, Mpv};

use crate::{server::FetchResult, ui::App, utils::season_or_specials_label, widgets::breadcrumbs};

use super::{Page, Pages};

pub struct Episode {
    pub id: i64,
}

impl Page for Episode {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(app: &mut App, ui: &mut Ui, mpv: &Mpv) {
        let id = app.get_page_state::<Self>().id;
        let episode = match app.client.get_episode(id) {
            FetchResult::Loading => {
                ui.label("Loading..");
                return;
            }
            FetchResult::Error(msg) => {
                ui.label(msg);
                return;
            }
            FetchResult::Ok(episode) => episode,
        };
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

        let show = shows
            .iter()
            .find(|s| s.id == episode.episode_file.series_id)
            .unwrap()
            .to_owned();

        if breadcrumbs(
            ui,
            vec![
                "Overview".into(),
                format!(
                    "{} ({})",
                    show.title,
                    season_or_specials_label(episode.season_number)
                ),
                format!("{} (Episode {})", episode.title, episode.episode_number),
            ],
        )
        .clicked()
        {
            app.navigate(Pages::Show {
                id: show.id,
                season: episode.episode_file.season_number,
            });
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
                    let episode_banner_url = episode
                        .images
                        .iter()
                        .find(|image| image.cover_type == "screenshot");

                    if let Some(image) = app
                        .network_image_cache
                        .fetch_image(episode_banner_url.unwrap().url.clone())
                    {
                        image.show_max_size(ui, Vec2::new(400.0, 500.0));
                    } else {
                        ui.allocate_exact_size(Vec2::new(400.0, 500.0), Sense::click());
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
                                    ui.heading(episode.title);
                                    if let Some(overview) = episode.overview {
                                        ui.label(RichText::new(overview));
                                    }
                                    ui.label(episode.episode_file.media_info.run_time);

                                    if ui.button("Play").clicked() {
                                        let watch_url = format!(
                                            "{}/episodes/{}/watch",
                                            app.config.read().unwrap().server_address,
                                            episode.id
                                        );
                                        println!("play episode: {watch_url}");
                                        app.navigate(Pages::Player);
                                        mpv.playlist_load_files(&[(
                                            &watch_url,
                                            FileState::AppendPlay,
                                            None,
                                        )])
                                        .unwrap();
                                    }
                                });
                            });
                    });
                });
            });
    }
}
