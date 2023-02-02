use std::collections::HashMap;

use egui::Vec2;
use egui::{FontFamily, FontId, TextStyle};
use egui_glow::egui_winit::winit::event::{ElementState, VirtualKeyCode, WindowEvent};
use libmpv::events::PropertyData;
use libmpv::{FileState, Mpv, MpvNode};

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

    style.visuals.window_fill = egui::Color32::from_rgb(15, 23, 42);

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

#[derive(Debug)]
struct MpvProperties {
    pub duration: f64,
    pub time_pos: f64,
    pub seekable_ranges: Vec<(f64, f64)>,
    pub playback: bool,
    pub is_paused: bool,
}

impl Default for MpvProperties {
    fn default() -> Self {
        Self {
            duration: 0.0,
            time_pos: 0.0,
            seekable_ranges: vec![(0.0, 0.0)],
            playback: false,
            is_paused: false,
        }
    }
}

pub struct App {
    filepath: String,
    prev_seek: f32,
    properties: MpvProperties,
}

impl App {
    pub fn new(ctx: &egui::Context) -> Self {
        // setup egui styles
        configure_text_styles(ctx);
        configure_default_button(ctx);

        Self { 
            filepath: String::from("https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/1080/Big_Buck_Bunny_1080_10s_5MB.mp4"),
            properties: MpvProperties::default(),
            prev_seek: 0.0,
        }
    }

    pub fn player_ui(&mut self, ctx: &egui::Context, mpv: &Mpv) {
        egui::Area::new("controls")
            .anchor(egui::Align2::LEFT_BOTTOM, Vec2::new(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    {                   
                        let playbar = ui.add(widgets::playbar::Playbar::new(
                            self.properties.duration,
                            self.properties.time_pos,
                            self.properties.seekable_ranges.clone(),
                        ));

                        if playbar.clicked() || playbar.dragged() {
                            let pos = playbar.interact_pointer_pos().unwrap();
                            let seek_to = (pos.x) / ui.available_width() * self.properties.duration as f32;
                            if self.prev_seek != seek_to {
                                mpv.seek_absolute(seek_to as f64).unwrap();
                                mpv.pause().unwrap();
                                self.prev_seek = seek_to;
                            }
                        }

                        if playbar.drag_released() {
                            self.prev_seek = 0.0;
                            mpv.unpause().unwrap();
                        }
                    }

                    if ui
                        .button(if self.properties.is_paused { "Play" } else { "Pause" })
                        .clicked()
                    {
                        mpv.cycle_property("pause", true).unwrap();
                    }

                    if ui.button("Stop").clicked() {
                        mpv.playlist_remove_current().unwrap();
                    }
                });
            });
    }

    pub fn render(&mut self, ctx: &egui::Context, mpv: &Mpv) {
        egui::CentralPanel::default()
            .frame(if self.properties.playback {
                egui::Frame::none()
            } else {
                egui::Frame::none().fill(ctx.style().visuals.window_fill)
            })
            .show(ctx, |ui| {
                egui::Frame::none()
                    .inner_margin(20.0)
                    .outer_margin(0.0)
                    .show(ui, |ui| {
                        if self.properties.playback {
                            self.player_ui(ctx, mpv);
                            return;
                        }

                        ui.heading("Playarr");

                        ui.text_edit_singleline(&mut self.filepath);

                        if ui.button("Watch").clicked() {
                            mpv.playlist_load_files(&[(
                                &self.filepath,
                                FileState::AppendPlay,
                                None,
                            )])
                            .unwrap();
                        }
                    })
            });
    }

    pub fn handle_player_keyboard_events(&mut self, event: &WindowEvent, mpv: &Mpv) {
        if !self.properties.playback {
            return;
        }

        if let WindowEvent::KeyboardInput {
            device_id: _,
            input,
            is_synthetic: _,
        } = event
        {
            if input.virtual_keycode.is_none() {
                return;
            }
            if input.state != ElementState::Released {
                return;
            }

            // is shift held
            let seek_time = if input.modifiers.shift() { 1.0 } else { 5.0 };

            match input.virtual_keycode.unwrap() {
                VirtualKeyCode::Left => mpv.seek_backward(seek_time).unwrap(),
                VirtualKeyCode::Right => {
                    mpv.seek_forward(seek_time).unwrap();
                }
                VirtualKeyCode::Space => {
                    mpv.cycle_property("pause", true).unwrap();
                }
                _ => {}
            }
        }
    }

    pub fn handle_mpv_events(&mut self, event: &libmpv::events::Event) {
        match event {
            libmpv::events::Event::PlaybackRestart => {
                self.properties.playback = true;
                self.properties.is_paused = false;
            }
            libmpv::events::Event::EndFile(_) => {
                self.properties.playback = false;
                self.properties.is_paused = false;
            }
            libmpv::events::Event::PropertyChange {
                name,
                change,
                reply_userdata: _,
            } => {
                if name == &"time-pos" {
                    let PropertyData::Double(time_pos) = change else {
                        return;
                    };
                    self.properties.time_pos = *time_pos;
                }
                if name == &"pause" {
                    let PropertyData::Flag(is_paused) = change else {
                        return;
                    };
                    self.properties.is_paused = *is_paused;
                }
                if name == &"duration" {
                    let PropertyData::Double(duration) = change else {
                        return;
                    };
                    self.properties.duration = *duration;
                }
                if name == &"demuxer-cache-state" {
                    let PropertyData::Node(mpv_node) = change else {
                        return;
                    };
                    let seekable_ranges = |node: &MpvNode| {
                        let mut res = Vec::new();
                        let props: HashMap<&str, MpvNode> = node.to_map()?.collect();
                        let ranges = props.get("seekable-ranges")?.to_array()?;

                        for node in ranges {
                            let range: HashMap<&str, MpvNode> = node.to_map()?.collect();
                            let start = range.get("start")?.to_f64()?;
                            let end = range.get("end")?.to_f64()?;
                            res.push((start, end));
                        }

                        Some(res)
                    };
                    self.properties.seekable_ranges = seekable_ranges(mpv_node).unwrap();
                }

                // println!("PropertyChange: {} {:?} {:?}", name, change, reply_userdata)
            }
            _ => {}
        }
    }
}
