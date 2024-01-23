use app::model::LostThoughts;
use iced::{Application, Font, Settings};
pub mod api;
pub mod app;

fn main() -> iced::Result {
    LostThoughts::run(Settings {
        default_font: Font::MONOSPACE,
        default_text_size: 30.0,
        ..Settings::default()
    })
}
