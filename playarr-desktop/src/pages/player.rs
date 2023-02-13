use crate::{
    ui::App,
    utils::seconds_to_video_duration,
    widgets::{icons::*, Playbar, VolumeControl},
};
use egui::*;
use egui::{style::Margin, Ui};
use libmpv::Mpv;

use super::Page;

#[derive(Clone)]
pub struct Player {
    pub prev_seek: f64,
}

impl Default for Player {
    fn default() -> Self {
        Self { prev_seek: 0.0 }
    }
}

impl Page for Player {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(app: &mut App, ui: &mut Ui, mpv: &Mpv) {
        if app
            .state
            .timestamp_last_mouse_movement
            .elapsed()
            .as_secs_f32()
            >= 1.5
        {
            return;
        }

        egui::TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .frame(Frame::none().inner_margin(10.0))
            .show(ui.ctx(), |ui| {
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
            .show(ui.ctx(), |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(15.0, 0.0);

                // seek bar
                {
                    let mut seek_to = app.get_page_state::<Self>().prev_seek;
                    let playbar = ui.add(Playbar::new(
                        app.properties.duration,
                        app.properties.time_pos,
                        app.properties.seekable_ranges.clone(),
                        &mut seek_to,
                    ));

                    if seek_to != app.get_page_state::<Self>().prev_seek {
                        app.get_page_state_mut::<Self>().prev_seek = seek_to;
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

                            /*
                             * Left column of the playbar
                             */
                            {
                                let (rect, _) = ui.allocate_exact_size(
                                    Vec2::new(left_column, 20.0),
                                    Sense::click(),
                                );
                                ui.child_ui(rect, *ui.layout()).label(format!(
                                    "{:02} / {:02}",
                                    seconds_to_video_duration(app.properties.time_pos),
                                    seconds_to_video_duration(app.properties.duration)
                                ));
                            }

                            /*
                             * Middle column
                             */
                            {
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
                                    if app.properties.is_paused {
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
                            }

                            /*
                             * Right column of the playbar
                             */
                            {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if icon(
                                        ui,
                                        if app.state.is_fullscreen {
                                            &MINIMIZE_ICON
                                        } else {
                                            &MAXIMIZE_ICON
                                        },
                                    )
                                    .clicked()
                                    {
                                        app.el_proxy
                                            .send_event(crate::UserEvent::ToggleFullscreen)
                                            .unwrap_or_default();
                                    }
                                    ui.add_space(10.0);

                                    let mut volume_control = app.properties.volume;
                                    ui.add(VolumeControl::new(&mut volume_control));
                                    if volume_control != app.properties.volume {
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
                            }
                        });
                    });
            });
    }
}
