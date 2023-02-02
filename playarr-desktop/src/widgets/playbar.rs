use std::time::Duration;

use egui::*;

pub struct Playbar {
    width: f32,
    duration: f64,
    pos: f64,
    seekable_ranges: Vec<(f64, f64)>,
}

impl Playbar {
    pub fn new(width: f32, duration: f64, pos: f64, seekable_ranges: Vec<(f64, f64)>) -> Self {
        Self {
            duration,
            pos,
            width,
            seekable_ranges,
        }
    }
}

impl Widget for Playbar {
    fn ui(self, ui: &mut Ui) -> Response {
        let Playbar {
            pos,
            duration,
            width,
            seekable_ranges,
        } = self;

        let height = 10.0;
        let (rect, response) =
            ui.allocate_at_least(Vec2::new(width, height), Sense::click_and_drag());

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
                    let duration = duration * percentage as f64;
    
                    egui::Area::new("seek_text")
                        .fixed_pos(Pos2::new(hover_pos.x - 20.0, painter.round_to_pixel(rect.top() - 35.0)))
                        .constrain(true)
                        .interactable(false)
                        .show(ui.ctx(), |ui: &mut Ui| {
                            egui::Frame::none().inner_margin(5.0).rounding(5.0).fill(ui.visuals().window_fill).show(ui, |ui| {
                                ui.label(
                                    print_seconds_nice(duration)
                                )
                            })
                        });
                }
            }
        }

        response
    }
}

fn print_seconds_nice(seconds: f64) -> String {
    let duration = chrono::Duration::from_std(Duration::from_secs(seconds as u64)).unwrap();
    let seconds_padded = format!("{:02}", duration.num_seconds() % 60);
    format!("{}:{}", duration.num_minutes(), seconds_padded)
}
