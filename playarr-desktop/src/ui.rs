use std::collections::HashMap;
use std::time::{Duration, Instant};

use egui::style::Margin;
use egui::{Align, Color32, Frame, Layout, Sense, Ui, Vec2};
use egui::{FontFamily, FontId, TextStyle};
use egui_glow::egui_winit::winit::event::{ElementState, VirtualKeyCode, WindowEvent};
use libmpv::events::PropertyData;
use libmpv::{FileState, Mpv, MpvNode};
use winit::event::MouseScrollDelta;

use crate::widgets;
use crate::widgets::icons::{
    icon, CHEVRON_LEFT_ICON, PAUSE_ICON, PLAY_ICON, SEEK_BACK_ICON, SEEK_FORWARD_ICON,
    VOLUME_MAX_ICON, VOLUME_MUTE_ICON,
};
use crate::widgets::volume::VolumeControl;

#[derive(Debug)]
struct MpvProperties {
    pub duration: f64,
    pub time_pos: f64,
    pub seekable_ranges: Vec<(f64, f64)>,
    pub playback: bool,
    pub is_paused: bool,
    pub volume: i64,
}

impl Default for MpvProperties {
    fn default() -> Self {
        Self {
            duration: 0.0,
            time_pos: 0.0,
            seekable_ranges: vec![(0.0, 0.0)],
            playback: false,
            is_paused: false,
            volume: 100,
        }
    }
}

pub struct App {
    filepath: String,
    timestamp_last_mouse_movement: Instant,
    properties: MpvProperties,
    prev_seek: f64,
}

impl App {
    pub fn new(ctx: &egui::Context) -> Self {
        // setup egui styles
        configure_text_styles(ctx);
        configure_default_button(ctx);

        Self {
            filepath: String::from("https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/1080/Big_Buck_Bunny_1080_10s_5MB.mp4"),
            timestamp_last_mouse_movement: std::time::Instant::now(),
            properties: MpvProperties::default(),
            prev_seek: 0.0,
        }
    }

    pub fn player_ui(&mut self, ctx: &egui::Context, mpv: &Mpv) {
        if self.timestamp_last_mouse_movement.elapsed().as_secs_f32() >= 1.5 {
            return;
        }

        egui::TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .frame(Frame::none().inner_margin(10.0))
            .show(ctx, |ui| {
                if icon(ui, &CHEVRON_LEFT_ICON).clicked() {
                    mpv.playlist_remove_current().unwrap();
                }
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(
                Frame::none()
                    .inner_margin(0.0)
                    .fill(Color32::from_black_alpha(150)),
            )
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(15.0, 0.0);

                // seek bar
                {
                    let mut seek_to = self.prev_seek;
                    let playbar = ui.add(widgets::playbar::Playbar::new(
                        self.properties.duration,
                        self.properties.time_pos,
                        self.properties.seekable_ranges.clone(),
                        &mut seek_to,
                    ));

                    if seek_to != self.prev_seek {
                        self.prev_seek = seek_to;
                        mpv.pause().unwrap();
                        mpv.seek_absolute(seek_to).unwrap();
                    }
                    if playbar.drag_released() {
                        mpv.seek_absolute(seek_to).unwrap();
                        mpv.unpause().unwrap();
                    }
                }

                egui::Frame::none()
                    .inner_margin(Margin {
                        left: 20.0,
                        right: 20.0,
                        top: 10.0,
                        bottom: 10.0,
                    })
                    .show(ui, |ui: &mut Ui| {
                        ui.horizontal_centered(|ui: &mut Ui| {
                            let initial_avail_width = ui.available_width();

                            let left_column = 150.0;
                            let (rect, _) = ui
                                .allocate_exact_size(Vec2::new(left_column, 20.0), Sense::click());
                            ui.child_ui(rect, *ui.layout()).label(format!(
                                "{:02} / {:02}",
                                seconds_to_video_duration(self.properties.time_pos),
                                seconds_to_video_duration(self.properties.duration)
                            ));

                            let icon_size = 20.0;
                            let icon_amount = 3.0;
                            let total_gap = ui.spacing().item_spacing.x * icon_amount;

                            ui.add_space(
                                (initial_avail_width / 2.0)
                                    - ((icon_size * icon_amount + total_gap) / 2.0)
                                    - (left_column + ui.spacing().item_spacing.x),
                            );

                            if icon(ui, &SEEK_BACK_ICON).clicked() {
                                mpv.seek_backward(10.0).unwrap();
                            }
                            if icon(
                                ui,
                                if self.properties.is_paused {
                                    &PLAY_ICON
                                } else {
                                    &PAUSE_ICON
                                },
                            )
                            .clicked()
                            {
                                mpv.cycle_property("pause", true).unwrap();
                            }
                            if icon(ui, &SEEK_FORWARD_ICON).clicked() {
                                mpv.seek_forward(10.0).unwrap();
                            }

                            // right column
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                let mut volume_control = self.properties.volume;
                                ui.add(VolumeControl::new(&mut volume_control));
                                if volume_control != self.properties.volume {
                                    mpv.set_property("volume", volume_control).unwrap();
                                }
                                if icon(
                                    ui,
                                    if volume_control == 0 {
                                        &VOLUME_MUTE_ICON
                                    } else {
                                        &VOLUME_MAX_ICON
                                    },
                                )
                                .clicked()
                                {
                                    mpv.set_property(
                                        "volume",
                                        if volume_control == 0 { 100 } else { 0 },
                                    )
                                    .unwrap();
                                }
                            });
                        });
                    });
            });
    }

    pub fn render(&mut self, ctx: &egui::Context, mpv: &Mpv) {
        let body = egui::CentralPanel::default()
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
                            self.timestamp_last_mouse_movement = std::time::Instant::now();
                            self.properties.playback = true;
                            mpv.playlist_load_files(&[(
                                &self.filepath,
                                FileState::AppendPlay,
                                None,
                            )])
                            .unwrap();
                        }
                    })
            });

        if body.response.interact(Sense::click()).clicked() {
            self.on_body_click(mpv)
        }
    }

    pub fn on_body_click(&mut self, mpv: &Mpv) {
        if self.properties.playback {
            mpv.cycle_property("pause", true).unwrap();
        }
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

    pub fn handle_player_mouse_events(&mut self, event: &WindowEvent, mpv: &Mpv) {
        if !self.properties.playback {
            return;
        }

        match event {
            WindowEvent::CursorMoved {
                device_id: _,
                position: _,
                modifiers: _,
            } => {
                self.timestamp_last_mouse_movement = std::time::Instant::now();
            }
            WindowEvent::MouseWheel {
                device_id: _,
                delta,
                phase: _,
                modifiers: _,
            } => {
                if let MouseScrollDelta::LineDelta(_, y) = delta {
                    let curr_volume = mpv.get_property::<i64>("volume").unwrap();
                    if *y > 0.0 {
                        mpv.set_property("volume", curr_volume + 5).unwrap();
                    }
                    if *y < 0.0 && curr_volume != 0 {
                        mpv.set_property(
                            "volume",
                            match curr_volume {
                                0..=4 => 0,
                                _ => curr_volume - 5,
                            },
                        )
                        .unwrap();
                    }
                }
            }
            _ => {}
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
                if name == &"volume" {
                    let PropertyData::Int64(volume) = change else {
                        return;
                    };
                    self.properties.volume = *volume;
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

pub fn seconds_to_video_duration(seconds: f64) -> String {
    let duration = chrono::Duration::from_std(Duration::from_secs(seconds as u64)).unwrap();
    let seconds_padded = format!("{:02}", duration.num_seconds() % 60);
    let minutes_padded = format!("{:02}", duration.num_minutes() % 60);
    if duration.num_hours() > 0 {
        return format!(
            "{}:{}:{}",
            duration.num_hours(),
            minutes_padded,
            seconds_padded
        );
    }

    format!("{}:{}", duration.num_minutes(), seconds_padded)
}
