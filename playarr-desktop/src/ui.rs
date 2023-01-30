use std::sync::{Arc, Mutex, RwLock};

use egui::{FontFamily, FontId, RichText, TextStyle};
use egui::{Ui, Vec2};
use libmpv::{FileState, Mpv};

use crate::widgets;

fn configure_text_styles(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Inter-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!("./assets/fonts/Inter-Regular.ttf")),
    );
    fonts.font_data.insert(
        "Inter-SemiBold".to_owned(),
        egui::FontData::from_static(include_bytes!("./assets/fonts/Inter-SemiBold.ttf")),
    );
    fonts.font_data.insert(
        "Inter-Bold".to_owned(),
        egui::FontData::from_static(include_bytes!("./assets/fonts/Inter-Bold.ttf")),
    );

    fonts.families.insert(
        FontFamily::Name("Inter-Regular".into()),
        vec!["Inter-Regular".into()],
    );
    fonts.families.insert(
        FontFamily::Name("Inter-SemiBold".into()),
        vec!["Inter-SemiBold".into()],
    );
    fonts.families.insert(
        FontFamily::Name("Inter-Bold".into()),
        vec!["Inter-Bold".into()],
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Inter-Regular".to_owned());

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(32.0, FontFamily::Name("Inter-Bold".into())),
        ),
        (TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(18.0, FontFamily::Name("Inter-SemiBold".into())),
        ),
        (TextStyle::Small, FontId::new(8.0, FontFamily::Proportional)),
        (
            TextStyle::Monospace,
            FontId::new(16.0, FontFamily::Proportional),
        ),
    ]
    .into();

    ctx.set_style(style);

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

fn configure_default_button(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // spacing
    style.spacing.button_padding = egui::vec2(20.0, 8.0);
    style.spacing.item_spacing = egui::vec2(0.0, 12.0);

    // default stae
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(99, 102, 241);

    // hovered widgets
    style.visuals.widgets.hovered.expansion = 0.0;
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(79, 70, 229);
    style.visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(0.0, egui::Color32::from_rgb(79, 70, 229));

    // active widgets
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(58, 48, 226);
    style.visuals.widgets.active.bg_stroke =
        egui::Stroke::new(0.0, egui::Color32::from_rgb(58, 48, 226));
    style.visuals.widgets.active.expansion = 0.0;

    style.visuals.override_text_color = Some(egui::Color32::from_rgb(255, 255, 255));

    ctx.set_style(style);
}

pub struct App {
    mpv: Arc<RwLock<Mpv>>,
    filepath: String,
    pub playback: bool,
    pub is_paused: bool,
    pub seek_pos: f64,
}

impl App {
    pub fn new(mpv: Arc<RwLock<Mpv>>, ctx: &egui::Context) -> Self {
        // setup egui styles
        configure_text_styles(ctx);
        configure_default_button(ctx);

        Self { mpv, filepath: String::from("https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/1080/Big_Buck_Bunny_1080_10s_5MB.mp4"), playback: false, is_paused: false, seek_pos: 0.0 }
    }

    pub fn player_ui(&mut self, ctx: &egui::Context) {
        egui::Area::new("controls")
            .anchor(egui::Align2::LEFT_BOTTOM, Vec2::new(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    let duration = self
                        .mpv
                        .read()
                        .unwrap()
                        .get_property::<i64>("duration")
                        .unwrap_or(0);

                    let time_pos = self
                        .mpv
                        .read()
                        .unwrap()
                        .get_property::<i64>("time-pos")
                        .unwrap_or(0);

                    let size = 1024.0;
                    let playbar =
                        ui.add(widgets::playbar::Playbar::new(1024.0, duration, time_pos));

                    if playbar.clicked() {
                        let pos = playbar.interact_pointer_pos().unwrap();
                        let seek_to = (pos.x) / size * duration as f32;
                        self.mpv
                            .read()
                            .unwrap()
                            .seek_absolute(seek_to as f64)
                            .unwrap();
                    }

                    if ui
                        .button(if self.is_paused { "Play" } else { "Pause" })
                        .clicked()
                    {
                        if self.is_paused {
                            self.mpv.read().unwrap().unpause().unwrap();
                        } else {
                            self.mpv.read().unwrap().pause().unwrap();
                        }
                        self.is_paused = !self.is_paused;
                    }

                    if ui.button("Stop").clicked() {
                        self.mpv.read().unwrap().playlist_remove_current().unwrap();
                    }
                });
            });
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(if self.playback {
                egui::Frame::none()
            } else {
                egui::Frame::none().fill(egui::Color32::from_rgb(15, 23, 42))
            })
            .show(ctx, |ui| {
                egui::Frame::none()
                    .inner_margin(20.0)
                    .outer_margin(0.0)
                    .show(ui, |ui| {
                        if self.playback {
                            self.player_ui(ctx);
                            return;
                        }

                        ui.heading("Playarr");

                        ui.text_edit_singleline(&mut self.filepath);

                        if ui.button("Watch").clicked() {
                            self.mpv
                                .read()
                                .unwrap()
                                .playlist_load_files(&[(
                                    &self.filepath,
                                    FileState::AppendPlay,
                                    None,
                                )])
                                .unwrap();
                        }
                    })
            });
    }
}
