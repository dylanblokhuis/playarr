use egui::*;

pub struct Playbar {
    width: f32,
    duration: i64,
    pos: i64,
}

impl Playbar {
    pub fn new(width: f32, duration: i64, pos: i64) -> Self {
        Self {
            duration,
            pos,
            width,
        }
    }
}

impl Widget for Playbar {
    fn ui(self, ui: &mut Ui) -> Response {
        let Playbar {
            pos,
            duration,
            width,
        } = self;

        let height = 10.0;
        let (rect, response) =
            ui.allocate_at_least(Vec2::new(width, height), Sense::click_and_drag());

        if ui.is_rect_visible(response.rect) {
            let stroke = ui.visuals().widgets.active.bg_stroke;
            let painter = ui.painter();
            painter.hline(
                rect.x_range(),
                painter.round_to_pixel(rect.center().y),
                Stroke::new(height, Color32::from_rgb(0, 0, 0)),
            );
            painter.hline(
                0.0..=pos as f32 / duration as f32 * rect.width(),
                painter.round_to_pixel(rect.center().y),
                Stroke::new(height, stroke.color),
            )
        }
        response
    }
}
