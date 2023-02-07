use egui::{style::Margin, *};

use super::icons::{icon, CHEVRON_LEFT_ICON};

pub fn breadcrumbs(ui: &mut Ui, crumbs: Vec<String>) -> Response {
    let frame = egui::Frame::none()
        .inner_margin(Margin {
            top: 20.0,
            bottom: 0.0,
            left: 35.0,
            right: 35.0,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let back_icon = icon(ui, &CHEVRON_LEFT_ICON);
                ui.add_space(25.0);
                for (index, crumb) in crumbs.iter().enumerate() {
                    ui.label(RichText::new(crumb));
                    if index != crumbs.len() - 1 {
                        ui.add_space(10.0);
                        ui.label(RichText::new("/").size(18.0));
                        ui.add_space(10.0);
                    }
                }

                back_icon
            })
        });

    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    frame.inner.inner
}
