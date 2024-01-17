use iced::widget::{button, column, horizontal_space, row, text_input, vertical_space, Column};
use iced::{executor, Application, Font, Length, Theme};
use iced::{Command, Settings};

fn main() -> iced::Result {
    LostThoughts::run(Settings {
        default_font: Font::MONOSPACE,
        default_text_size: 30.0,
        ..Settings::default()
    })
}

struct LostThoughts {
    user: User,
    current_window: WindowState,
    logged_in: bool,
    debbug: bool,
    search: String,
}

struct User {
    login: String,
    password: String,
    email: String,
}

#[derive(Debug, Clone)]
struct Post {}

#[derive(Debug, Clone)]
enum WindowState {
    Login,
    Register,
    Main,
    Account,
    Poster,
    Search,
}

#[derive(Debug, Clone)]
enum Message {
    SignIn,
    SignUp,
    SwitchWindow(WindowState),
    PasswordChange(String),
    LoginChange(String),
    EmailChange(String),
    SearchChange(String),
    PostAdd(Vec<Post>),
    DebugSwitch(bool),
}

impl Application for LostThoughts {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                user: User {
                    login: String::new(),
                    password: String::new(),
                    email: String::new(),
                },
                logged_in: false,
                current_window: WindowState::Login,
                debbug: true,
                search: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Lost Thoughts".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::SignIn => Command::none(),
            Message::SignUp => Command::none(),
            Message::SwitchWindow(window) => {
                self.current_window = window;
                Command::none()
            }
            Message::PasswordChange(password) => {
                self.user.password = password;
                Command::none()
            }
            Message::LoginChange(login) => {
                self.user.login = login;
                Command::none()
            }
            Message::EmailChange(email) => {
                self.user.email = email;
                Command::none()
            }
            Message::SearchChange(value) => {
                self.search = value;
                Command::none()
            }
            Message::PostAdd(_) => todo!(),
            Message::DebugSwitch(value) => {
                self.debbug = value;
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match self.current_window {
            WindowState::Login => column![
                text_input("Login", &self.user.login)
                    .on_input(Message::LoginChange),
                text_input("Password", &self.user.password)
                    .password()
                    .on_input(Message::PasswordChange),
                row![
                    horizontal_space(Length::Fill),
                    button("Submit").on_press(Message::SignIn),
                    button("Has no account?")
                        .on_press(Message::SwitchWindow(WindowState::Register)),
                    horizontal_space(Length::Fill),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center)
            ]
            .padding(30)
            .spacing(20),
            WindowState::Register => column![
                text_input("Login", &self.user.login)
                    .on_input(Message::LoginChange),
                text_input("Password", &self.user.password)
                    .password()
                    .on_input(Message::PasswordChange),
                text_input("Confirm Password", &self.user.password),
                text_input("Email", &self.user.email)
                    .on_input(Message::EmailChange),
                row![
                    horizontal_space(Length::Fill),
                    button("Submit").on_press(Message::SignUp),
                    button("Already have an account?")
                        .on_press(Message::SwitchWindow(WindowState::Login)),
                    horizontal_space(Length::Fill),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center)
            ]
            .padding(30)
            .spacing(20),
            WindowState::Main => column![],
            WindowState::Account => column![],
            WindowState::Poster => column![],
            WindowState::Search => column![],
        };

        let debbug_menu = if self.debbug {
            row![
                horizontal_space(Length::Fill),
                button("login").on_press(Message::SwitchWindow(WindowState::Login)),
                button("register").on_press(Message::SwitchWindow(WindowState::Register)),
                button("main").on_press(Message::SwitchWindow(WindowState::Main)),
                button("account").on_press(Message::SwitchWindow(WindowState::Account)),
                button("poster").on_press(Message::SwitchWindow(WindowState::Poster)),
                button("search").on_press(Message::SwitchWindow(WindowState::Search)),
                horizontal_space(Length::Fill),
                button("debbug off").on_press(Message::DebugSwitch(false)),
            ]
            .align_items(iced::Alignment::Center)
            .spacing(10)
        } else {
            row![
                horizontal_space(Length::Fill),
                button("debbug on").on_press(Message::DebugSwitch(true)),
            ]
            .padding(30)
        };

        column![content, vertical_space(Length::Fill), debbug_menu]
            .spacing(10)
            .padding(10)
            .into()
    }
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
