use std::time::Duration;

use egui::*;

pub struct Playbar {
    duration: f64,
    pos: f64,
    seekable_ranges: Vec<(f64, f64)>,
}

impl Playbar {
    pub fn new(duration: f64, pos: f64, seekable_ranges: Vec<(f64, f64)>) -> Self {
        Self {
            duration,
            pos,
            seekable_ranges,
        }
    }
}

impl Widget for Playbar {
    fn ui(self, ui: &mut Ui) -> Response {
        let Playbar {
            pos,
            duration,
            seekable_ranges,
        } = self;

        let response = ui.vertical(|ui| {
            egui::Frame::none().show(ui, |ui| {
                ui.label("Hello");
            });

            ui.add(|ui: &mut Ui| {
                // the bar
                let height = 10.0;
                let (rect, response) = ui.allocate_at_least(
                    Vec2::new(ui.available_width(), height),
                    Sense::click_and_drag(),
                );

                if ui.is_rect_visible(response.rect) {
                    let stroke = ui.visuals().widgets.active.bg_stroke;
                    let painter = ui.painter();

                    // paint background
                    painter.hline(
                        rect.x_range(),
                        painter.round_to_pixel(rect.center().y),
                        Stroke::new(height, Color32::from_rgb(0, 0, 0)),
                    );

                    // paint seekable ranges
                    for (start, end) in seekable_ranges {
                        painter.hline(
                            start as f32 / duration as f32 * rect.width()
                                ..=end as f32 / duration as f32 * rect.width(),
                            painter.round_to_pixel(rect.center().y),
                            Stroke::new(
                                height,
                                Color32::from_rgba_unmultiplied(
                                    stroke.color.r(),
                                    stroke.color.g(),
                                    stroke.color.b(),
                                    80,
                                ),
                            ),
                        );
                    }

                    // paint current progress
                    let end_pos_line = pos as f32 / duration as f32 * rect.width();
                    painter.hline(
                        0.0..=end_pos_line,
                        painter.round_to_pixel(rect.center().y),
                        Stroke::new(height, stroke.color),
                    );

                    // circle to show there's handle
                    if response.hovered() || response.dragged() {
                        painter.circle_filled(
                            Pos2::new(end_pos_line, painter.round_to_pixel(rect.center().y)),
                            7.5,
                            Color32::WHITE,
                        );
                    }

                    // point of seeking text
                    if response.hovered() && response.hover_pos().is_some() {
                        let hover_pos = response.hover_pos().unwrap().x;
                        let percentage = hover_pos / rect.width();
                        let duration = duration * percentage as f64;

                        // painter.text(
                        //     Pos2::new(hover_pos, painter.round_to_pixel(rect.top() - 25.0)),
                        //     Align2::CENTER_CENTER,
                        //     print_seconds_nice(duration),
                        //     FontId::proportional(16.0),
                        //     Color32::WHITE,
                        // );
                    }
                }
                response
            })
        });
        response.inner
        // if response.hovered() {
        //     let hover_pos = response.hover_pos().unwrap().x;
        //     let percentage = hover_pos / ui.available_width();
        //     let duration = duration * percentage as f64;

        //     ui.add(|ui: &mut Ui| {
        //         egui::Area::new("pointer-timestamp")
        //             .fixed_pos(Pos2::new(hover_pos, ui.next_widget_position().y - 50.0))
        //             .show(ui.ctx(), |ui| {
        //                 egui::Frame::none()
        //                     .fill(egui::Color32::from_rgb(15, 23, 42))
        //                     .inner_margin(5.0)
        //                     .rounding(Rounding::default().at_least(5.0))
        //                     .show(ui, |ui| ui.label(print_seconds_nice(duration)))
        //                     .inner
        //             })
        //             .inner
        //     });
        // }
    }
}

fn print_seconds_nice(seconds: f64) -> String {
    let duration = chrono::Duration::from_std(Duration::from_secs(seconds as u64)).unwrap();
    let seconds_padded = format!("{:02}", duration.num_seconds() % 60);
    format!("{}:{}", duration.num_minutes(), seconds_padded)
}
