use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, RwLock};
use std::time::Instant;

use egui::{Color32, FontFamily, FontId, TextStyle};
use egui::{Sense, Stroke};
use egui_glow::egui_winit::winit::event::{ElementState, VirtualKeyCode, WindowEvent};
use libmpv::events::PropertyData;
use libmpv::{Mpv, MpvNode};
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;
use winit::event::MouseScrollDelta;

use crate::pages::{self, Page, Pages};
use crate::server::{Client, NetworkCache, NetworkImageCache};

#[derive(Debug)]
pub struct MpvProperties {
    pub duration: f64,
    pub time_pos: f64,
    pub seekable_ranges: Vec<(f64, f64)>,
    pub is_paused: bool,
    pub volume: i64,
}

impl Default for MpvProperties {
    fn default() -> Self {
        Self {
            duration: 0.0,
            time_pos: 0.0,
            seekable_ranges: vec![(0.0, 0.0)],
            is_paused: false,
            volume: 100,
        }
    }
}

pub struct AppState {
    page: Pages,
    page_state: Box<dyn Page>,
    pub timestamp_last_mouse_movement: Instant,
    pub prev_seek: f64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub server_address: String,
}

pub struct App {
    pub properties: MpvProperties,
    pub state: AppState,
    pub client: Client,
    pub network_image_cache: NetworkImageCache,
    pub config: Arc<RwLock<Config>>,
}

impl App {
    pub fn new(ctx: &egui::Context) -> Self {
        // setup egui styles
        configure_text_styles(ctx);
        configure_widgets(ctx);

        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        let network_cache = NetworkCache::new(rt, ctx.clone());
        let config = if let Some(config) = load_config() {
            config
        } else {
            Config {
                server_address: String::new(),
            }
        };

        let mut page = Pages::Onboarding;
        if !config.server_address.is_empty() {
            page = Pages::Overview;
        }
        let page_state = page.get_default_state();
        let config = Arc::new(RwLock::new(config));

        Self {
            state: AppState {
                page,
                page_state,
                timestamp_last_mouse_movement: std::time::Instant::now(),
                prev_seek: 0.0,
            },
            properties: MpvProperties::default(),
            client: Client::new(config.clone(), network_cache.clone()),
            network_image_cache: NetworkImageCache::new(network_cache, ctx.clone()),
            config,
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, mpv: &Mpv) {
        let body = egui::CentralPanel::default()
            .frame(if self.state.page == Pages::Player {
                egui::Frame::none()
            } else {
                egui::Frame::none().fill(ctx.style().visuals.window_fill)
            })
            .show(ctx, |ui| {
                egui::Frame::none()
                    .inner_margin(0.0)
                    .outer_margin(0.0)
                    .show(ui, |ui| match self.state.page {
                        Pages::Onboarding => pages::Onboarding::render(self, ui, mpv),
                        Pages::Overview => pages::Overview::render(self, ui, mpv),
                        Pages::Player => pages::Player::render(self, ui, mpv),
                        Pages::Show { id: _, season: _ } => pages::Show::render(self, ui, mpv),
                        Pages::Episode { id: _ } => pages::Episode::render(self, ui, mpv),
                    })
            });

        if body.response.interact(Sense::click()).clicked() {
            self.on_body_click(mpv)
        }
    }

    pub fn get_page(&self) -> Pages {
        self.state.page.clone()
    }

    pub fn navigate(&mut self, page: Pages) {
        self.state.page = page.clone();
        self.state.page_state = page.get_default_state();
    }

    pub fn on_body_click(&mut self, mpv: &Mpv) {
        if self.state.page == Pages::Player {
            mpv.cycle_property("pause", true).unwrap();
        }
    }

    pub fn handle_player_keyboard_events(&mut self, event: &WindowEvent, mpv: &Mpv) {
        if self.state.page != Pages::Player {
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
        if self.state.page != Pages::Player {
            return;
        }

        match event {
            WindowEvent::CursorMoved {
                device_id: _,
                position: _,
                modifiers: _,
            } => {
                self.state.timestamp_last_mouse_movement = std::time::Instant::now();
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
                self.state.page = Pages::Player;
                self.properties.is_paused = false;
            }
            libmpv::events::Event::EndFile(_) => {
                self.state.page = Pages::Overview;
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

    pub fn get_page_state_mut<T>(&mut self) -> &mut T
    where
        T: 'static,
    {
        self.state
            .page_state
            .as_any()
            .downcast_mut::<T>()
            .expect("Downcasted wrong type")
    }
    pub fn get_page_state<T>(&mut self) -> &T
    where
        T: 'static,
    {
        self.state
            .page_state
            .as_any()
            .downcast_ref::<T>()
            .expect("Downcasted wrong type")
    }

    pub fn save_config_to_disk(&self) {
        let config = self.config.read().unwrap();
        let config = toml::to_string(&config.clone()).unwrap();
        let mut file = std::fs::File::create("playarr.toml").unwrap();
        file.write_all(config.as_bytes()).unwrap();
    }
}

fn load_config() -> Option<Config> {
    let Ok(mut file) = std::fs::File::open("playarr.toml") else {
        return None
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Some(toml::from_str::<Config>(&contents).unwrap())
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
            FontId::new(16.0, FontFamily::Name("Inter-SemiBold".into())),
        ),
        (TextStyle::Small, FontId::new(8.0, FontFamily::Proportional)),
        (
            TextStyle::Monospace,
            FontId::new(16.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("SemiBold".into()),
            FontId::new(16.0, FontFamily::Name("Inter-SemiBold".into())),
        ),
        (
            TextStyle::Name("Bold".into()),
            FontId::new(16.0, FontFamily::Name("Inter-Bold".into())),
        ),
    ]
    .into();

    ctx.set_style(style);

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

fn configure_widgets(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // spacing
    style.spacing.button_padding = egui::vec2(15.0, 5.0);
    style.spacing.item_spacing = egui::vec2(0.0, 12.0);

    style.visuals.window_fill = Color32::from_rgb(15, 23, 42);

    // default stae
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(99, 102, 241);

    // hovered widgets
    style.visuals.widgets.hovered.expansion = 0.0;
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(79, 70, 229);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(0.0, Color32::from_rgb(79, 70, 229));

    // active widgets
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(58, 48, 226);
    style.visuals.widgets.active.bg_stroke = Stroke::new(0.0, Color32::from_rgb(58, 48, 226));
    style.visuals.widgets.active.expansion = 0.0;

    style.visuals.override_text_color = Some(Color32::from_rgb(255, 255, 255));

    style.visuals.widgets.noninteractive.bg_stroke =
        Stroke::new(1.0, Color32::from_rgb(55, 65, 81));

    ctx.set_style(style);
}
