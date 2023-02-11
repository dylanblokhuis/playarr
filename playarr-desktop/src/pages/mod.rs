mod episode;
mod onboarding;
mod overview;
mod player;
mod show;

use std::any::Any;

use egui::Ui;
pub use episode::Episode;
use libmpv::Mpv;
pub use onboarding::Onboarding;
pub use overview::Overview;
pub use player::Player;
pub use show::Show;

use crate::ui::App;

pub trait Page {
    fn render(app: &mut App, ui: &mut Ui, mpv: &Mpv)
    where
        Self: Sized;
    fn as_any(&mut self) -> &mut dyn Any;
}

#[derive(PartialEq, Clone)]
pub enum Pages {
    Onboarding,
    Overview,
    Player,
    Show { id: i64, season: i64 },
    Episode { id: i64 },
}

impl Pages {
    pub fn get_default_state(&self) -> Box<dyn Page> {
        match self {
            Pages::Overview => Box::new(Overview),
            Pages::Onboarding => Box::<Onboarding>::default(),
            Pages::Show { id, season } => Box::new(Show {
                id: *id,
                season: *season,
            }),
            Pages::Episode { id } => Box::new(Episode { id: *id }),
            Pages::Player => Box::new(Player),
        }
    }
}
