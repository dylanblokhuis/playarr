use egui::{Response, Sense, Ui};
use egui_extras::RetainedImage;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PLAY_ICON: RetainedImage = egui_extras::RetainedImage::from_svg_bytes_with_size(
        "play.svg",
        include_bytes!("../assets/icons/play.svg"),
        egui_extras::image::FitTo::Size(20, 20)
    )
    .unwrap();
    pub static ref PAUSE_ICON: RetainedImage =
        egui_extras::RetainedImage::from_svg_bytes_with_size(
            "pause.svg",
            include_bytes!("../assets/icons/pause.svg"),
            egui_extras::image::FitTo::Size(20, 20)
        )
        .unwrap();
    pub static ref SEEK_BACK_ICON: RetainedImage =
        egui_extras::RetainedImage::from_svg_bytes_with_size(
            "seek-back.svg",
            include_bytes!("../assets/icons/seek-back.svg"),
            egui_extras::image::FitTo::Size(20, 20)
        )
        .unwrap();
    pub static ref SEEK_FORWARD_ICON: RetainedImage =
        egui_extras::RetainedImage::from_svg_bytes_with_size(
            "seek-forward.svg",
            include_bytes!("../assets/icons/seek-forward.svg"),
            egui_extras::image::FitTo::Size(20, 20)
        )
        .unwrap();
    pub static ref CHEVRON_LEFT_ICON: RetainedImage =
        egui_extras::RetainedImage::from_svg_bytes_with_size(
            "chevron-left.svg",
            include_bytes!("../assets/icons/chevron-left.svg"),
            egui_extras::image::FitTo::Size(20, 20)
        )
        .unwrap();
    pub static ref VOLUME_MAX_ICON: RetainedImage =
        egui_extras::RetainedImage::from_svg_bytes_with_size(
            "volume-max.svg",
            include_bytes!("../assets/icons/volume-max.svg"),
            egui_extras::image::FitTo::Size(20, 20)
        )
        .unwrap();
    pub static ref VOLUME_MUTE_ICON: RetainedImage =
        egui_extras::RetainedImage::from_svg_bytes_with_size(
            "volume-mute.svg",
            include_bytes!("../assets/icons/volume-mute.svg"),
            egui_extras::image::FitTo::Size(20, 20)
        )
        .unwrap();
    pub static ref MAXIMIZE_ICON: RetainedImage =
        egui_extras::RetainedImage::from_svg_bytes_with_size(
            "maximize.svg",
            include_bytes!("../assets/icons/maximize.svg"),
            egui_extras::image::FitTo::Size(20, 20)
        )
        .unwrap();
    pub static ref MINIMIZE_ICON: RetainedImage =
        egui_extras::RetainedImage::from_svg_bytes_with_size(
            "minimize.svg",
            include_bytes!("../assets/icons/minimize.svg"),
            egui_extras::image::FitTo::Size(20, 20)
        )
        .unwrap();
}

pub fn icon(ui: &mut Ui, icon: &RetainedImage) -> Response {
    let res = egui::Frame::none()
        .fill(ui.visuals().widgets.active.bg_fill)
        .inner_margin(5.0)
        .rounding(100.0)
        .show(ui, |ui: &mut Ui| icon.show(ui))
        .response;

    if res.hovered() {
        ui.ctx().output().cursor_icon = egui::CursorIcon::PointingHand;
    }

    res.interact(Sense::click())
}
