use egui::*;

type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<i64>) -> i64>;

fn get(get_set_value: &mut GetSetValue<'_>) -> i64 {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: i64) {
    (get_set_value)(Some(value));
}

pub struct VolumeControl<'a> {
    volume: GetSetValue<'a>,
}

impl<'a> VolumeControl<'a> {
    pub fn new(volume: &'a mut i64) -> Self {
        Self {
            volume: Box::new(move |value| {
                if let Some(value) = value {
                    *volume = value;
                }
                *volume
            }),
        }
    }
}

impl<'a> Widget for VolumeControl<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(100.0, 15.0), Sense::click_and_drag());

        let painter = ui.painter();
        // painter.rect_filled(rect, Rounding::none(), Color32::RED);

        let slider_bg_y_range = painter.round_to_pixel(rect.center().y) - 2.5
            ..=painter.round_to_pixel(rect.center().y) + 2.5;

        // paint slider background
        painter.rect(
            Rect::from_x_y_ranges(rect.x_range(), slider_bg_y_range.clone()),
            Rounding::from(5.0),
            Color32::from_rgb(17, 24, 39),
            Stroke::NONE,
        );

        // paint slider front
        let x_rect_range = rect.x_range();
        let (start, end) = (x_rect_range.start(), x_rect_range.end());

        let volume = if get(&mut self.volume) > 100 {
            100
        } else {
            get(&mut self.volume)
        };
        let end = start + (end - start) * (volume as f32 / 100.0);

        painter.rect(
            Rect::from_x_y_ranges(*start..=end, slider_bg_y_range),
            Rounding::from(5.0),
            Color32::from_rgb(255, 255, 255),
            Stroke::NONE,
        );

        // paint slider handle
        painter.circle_filled(
            Pos2::new(end, painter.round_to_pixel(rect.center().y)),
            5.0,
            Color32::from_rgb(255, 255, 255),
        );

        if response.clicked() || response.dragged() {
            let x = response.interact_pointer_pos().unwrap().x;
            let volume = (x - rect.left()) / rect.width() * 100.0;

            let volume = if volume < 0.0 {
                0.0
            } else if volume > 100.0 {
                100.0
            } else {
                volume
            };

            set(&mut self.volume, volume as i64);
        }

        response
    }
}
