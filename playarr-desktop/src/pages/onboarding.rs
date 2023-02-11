use egui::{style::Margin, Color32, Frame, Label, Response, RichText, Sense, TextEdit, Ui, Vec2};
use libmpv::Mpv;

use crate::{server::FetchResult, ui::App};

use super::{Page, Pages};

#[derive(Default)]
pub struct Onboarding {
    pub try_request: bool,
    pub last_error: Option<String>,
}

impl Page for Onboarding {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(app: &mut App, ui: &mut Ui, mpv: &Mpv) {
        Frame::none().inner_margin(35.0).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Playarr");
                ui.label("Please fill out the following information to get started.");

                ui.add_space(10.0);

                let mut server_address = app.config.read().unwrap().server_address.clone();
                ui.add(
                    TextEdit::singleline(&mut server_address)
                        .hint_text(
                            RichText::new("Server Address")
                                .color(egui::Color32::from_rgb(148, 163, 184)),
                        )
                        .margin(Vec2::new(10.0, 5.0)),
                );
                ui.add_space(10.0);

                if server_address.is_empty() {
                    return;
                }

                if reqwest::Url::parse(&server_address).is_err() {
                    ui.label(
                        RichText::new("Please enter a valid URL.")
                            .color(Color32::from_rgb(220, 38, 38)),
                    );
                } else if ui.button("Save").clicked() {
                    app.get_page_state_mut::<Self>().try_request = true;
                }

                ui.add_space(10.0);

                if let Some(error) = &app.get_page_state::<Self>().last_error {
                    ui.label(
                        RichText::new("Can't connect with server:")
                            .color(Color32::from_rgb(220, 38, 38)),
                    );

                    ui.label(RichText::new(error).color(Color32::from_rgb(220, 38, 38)));
                }

                app.config.write().unwrap().server_address = server_address;

                if app.get_page_state_mut::<Self>().try_request {
                    match app.client.get_all_series() {
                        FetchResult::Loading => {
                            ui.label("Loading..");
                        }
                        FetchResult::Error(msg) => {
                            app.get_page_state_mut::<Self>().try_request = false;
                            app.get_page_state_mut::<Self>().last_error = Some(msg);
                        }
                        FetchResult::Ok(_) => {
                            app.save_config_to_disk();
                            app.navigate(Pages::Overview);
                        }
                    }
                }

                // ui.text_edit_singleline(&mut app.config.sonarr_address);
            });
        });
    }
}
