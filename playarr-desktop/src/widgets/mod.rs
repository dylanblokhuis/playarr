use egui::*;
use egui_extras::RetainedImage;

pub mod playbar;

pub fn icon(ui: &mut Ui, icon: &RetainedImage) -> Response {
    let res = egui::Frame::none()
        .fill(ui.visuals().widgets.active.bg_fill)
        .inner_margin(5.0)
        .rounding(100.0)
        .show(ui, |ui: &mut Ui| {
            icon.show(ui);
        })
        .response;

    res.interact(Sense::click())
}
