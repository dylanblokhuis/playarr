use egui::*;

use crate::utils::seconds_to_video_duration;

type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<f64>) -> f64>;

fn get(get_set_value: &mut GetSetValue<'_>) -> f64 {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: f64) {
    (get_set_value)(Some(value));
}

pub struct Playbar<'a> {
    duration: f64,
    pos: f64,
    seekable_ranges: Vec<(f64, f64)>,
    seek_to: GetSetValue<'a>,
}

impl<'a> Playbar<'a> {
    pub fn new(
        duration: f64,
        pos: f64,
        seekable_ranges: Vec<(f64, f64)>,
        seek_to: &'a mut f64,
    ) -> Self {
        Self {
            duration,
            pos,
            seekable_ranges,
            seek_to: Box::new(move |value| {
                if let Some(value) = value {
                    *seek_to = value;
                }
                *seek_to
            }),
        }
    }
}

impl<'a> Widget for Playbar<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
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
                Stroke::new(height, Color32::from_rgb(17, 24, 39)),
            );

            // paint seekable ranges
            for (start, end) in self.seekable_ranges {
                painter.hline(
                    start as f32 / self.duration as f32 * rect.width()
                        ..=end as f32 / self.duration as f32 * rect.width(),
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
            let end_pos_line = self.pos as f32 / self.duration as f32 * rect.width();
            painter.hline(
                0.0..=end_pos_line,
                painter.round_to_pixel(rect.center().y),
                Stroke::new(height, stroke.color),
            );

            if response.hovered() || response.dragged() {
                // circle to show there's handle
                painter.circle_filled(
                    Pos2::new(end_pos_line, painter.round_to_pixel(rect.center().y)),
                    7.5,
                    Color32::WHITE,
                );

                // seek text
                if let Some(hover_pos) = response.hover_pos() {
                    let percentage = hover_pos.x / rect.width();
                    let duration = self.duration * percentage as f64;

                    egui::Area::new("seek_text")
                        .fixed_pos(Pos2::new(
                            hover_pos.x - 20.0,
                            painter.round_to_pixel(rect.top() - 35.0),
                        ))
                        .constrain(true)
                        .interactable(false)
                        .show(ui.ctx(), |ui: &mut Ui| {
                            egui::Frame::none()
                                .inner_margin(5.0)
                                .rounding(5.0)
                                .fill(ui.visuals().window_fill)
                                .show(ui, |ui| {
                                    ui.add(
                                        Label::new(seconds_to_video_duration(duration)).wrap(false),
                                    )
                                })
                        });
                }

                // mutate seek_to

                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let seek_to = (pointer_pos.x) / ui.available_width() * self.duration as f32;
                    set(&mut self.seek_to, seek_to as f64);
                }
            }
        }

        response
    }
}
