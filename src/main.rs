use iced::widget::{column, row};
use iced::Settings;
use iced::{executor, Application, Font, Theme};

fn main() -> iced::Result {
    LostThoughts::run(Settings {
        default_font: Font::MONOSPACE,
        default_text_size: 30.0,
        ..Settings::default()
    })
}

struct LostThoughts {}

#[derive(Debug, Clone)]
enum Message {}

impl Application for LostThoughts {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {}, iced::Command::none())
    }

    fn title(&self) -> String {
        "Lost Thoughts".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {}
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let nothing = iced::widget::Text::new("Nothing here yet").size(30);
        let output = row![nothing];
        column![output,].spacing(10).padding(10).into()
    }
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
