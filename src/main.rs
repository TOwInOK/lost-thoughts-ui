use std::io::ErrorKind;
use serde::{Deserialize, Serialize};
use iced::widget::{button, column, horizontal_space, row, text_input, vertical_space, Column};
use iced::{executor, Application, Font, Length, Theme};
use iced::{Command, Settings};
use serde_json::json;

fn main() -> iced::Result {
    LostThoughts::run(Settings {
        default_font: Font::MONOSPACE,
        default_text_size: 30.0,
        ..Settings::default()
    })
}

#[derive(Clone)]
struct LostThoughts {
    user: User,
    current_window: WindowState,
    logged_in: bool,
    debbug: bool,
    search: String,
    title: String,
    posts: Vec<Post>,
}

#[derive(Clone, Serialize, Deserialize)]
struct User {
    login: String,
    password: String,
    email: String,
    #[serde(default = "default_role")]
    role: Role,
}

#[derive(Clone, Serialize, Deserialize)]
struct user_min {
    login: String,
    #[serde(default = "default_role")]
    role: Role,
}
    
fn default_role() -> Role {
    Role::Default
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Role {
    Admin,
    Paid,
    Default,
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
    SwitchWindow(WindowState, String),
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
                    role: Role::Default,
                },
                logged_in: false,
                current_window: WindowState::Login,
                debbug: true,
                search: String::new(),
                title: "Login".to_string(),
                posts: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::SignIn => (),
            Message::SignUp => (),
            Message::SwitchWindow(window, name_of_window) => {
                self.current_window = window;
                self.title = name_of_window;
            }
            Message::PasswordChange(password) => {
                self.user.password = password;
            }
            Message::LoginChange(login) => {
                self.user.login = login;
            }
            Message::EmailChange(email) => {
                self.user.email = email;
            }
            Message::SearchChange(value, ) => {
                self.search = value;
            }
            Message::PostAdd(_) => todo!(),
            Message::DebugSwitch(value) => {
                self.debbug = value;
            }
        }
        Command::none()
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
                        .on_press(Message::SwitchWindow(WindowState::Register, "Register".to_string())),
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
                        .on_press(Message::SwitchWindow(WindowState::Login, "Login".to_string())),
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
                button("login").on_press(Message::SwitchWindow(WindowState::Login, "Login".to_string())),
                button("register").on_press(Message::SwitchWindow(WindowState::Register, "Register".to_string())),
                button("main").on_press(Message::SwitchWindow(WindowState::Main, "Main".to_string())),
                button("account").on_press(Message::SwitchWindow(WindowState::Account, "Account".to_string())),
                button("poster").on_press(Message::SwitchWindow(WindowState::Poster, "Poster".to_string())),
                button("search").on_press(Message::SwitchWindow(WindowState::Search, "Search".to_string())),
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


async fn log_in(user: User) -> Result<(), ErrorKind> {
    todo!()
}
