use iced::widget::{button, column, horizontal_space, row, text_input, vertical_space};
use iced::{executor, Application, Font, Length, Theme};
use iced::{Command, Settings};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

fn main() -> iced::Result {
    LostThoughts::run(Settings {
        default_font: Font::MONOSPACE,
        default_text_size: 30.0,
        ..Settings::default()
    })
}

#[derive(Clone, Debug)]
struct LostThoughts {
    user: User,
    current_window: WindowState,
    logged_in: bool,
    debbug: bool,
    search: String,
    title: String,
    posts: Vec<Post>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct User {
    login: String,
    password: String,
    email: String,
    #[serde(default = "default_role")]
    role: Role,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct user_min {
    login: String,
    #[serde(default = "default_role")]
    role: Role,
}

fn default_role() -> Role {
    Role::Default
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    Signed(Result<StatusCode, String>),
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
            Message::SignIn => {
                Command::perform(log_in(self.user.clone()), |result| Message::Signed(result))
            }
            Message::SignUp => {
                Command::perform(log_in(self.user.clone()), |result| Message::Signed(result))
            }
            Message::SwitchWindow(window) => {
                match window {
                    WindowState::Login => {
                        self.current_window = window;
                        self.title = "Login".to_owned();
                    }
                    WindowState::Register => {
                        self.current_window = window;
                        self.title = "Register".to_owned();
                    }
                    WindowState::Main => {
                        self.current_window = window;
                        self.title = "Main".to_owned();
                    }
                    WindowState::Account => {
                        self.current_window = window;
                        self.title = "Account".to_owned();
                    }
                    WindowState::Poster => {
                        self.current_window = window;
                        self.title = "Poster".to_owned();
                    }
                    WindowState::Search => {
                        self.current_window = window;
                        self.title = "Search".to_owned();
                    }
                }

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
                println!("debbug is {}", value);
                self.debbug = value;
                Command::none()
            }
            Message::Signed(result) => match result {
                Ok(e) => match e {
                    StatusCode::OK => {
                        println!("{}", e);
                        Command::perform(async {}, |_| Message::SwitchWindow(WindowState::Main))
                    }
                    _ => {
                        println!("{}", e);
                        Command::none()
                    }
                },
                Err(e) => Command::none(),
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match self.current_window {
            WindowState::Login => column![
                text_input("Login", &self.user.login).on_input(Message::LoginChange),
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
                text_input("Login", &self.user.login).on_input(Message::LoginChange),
                text_input("Password", &self.user.password)
                    .password()
                    .on_input(Message::PasswordChange),
                text_input("Confirm Password", &self.user.password),
                text_input("Email", &self.user.email).on_input(Message::EmailChange),
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
async fn log_in(user: User) -> Result<StatusCode, String> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let client = Client::new();
            client
                .get(format!(
                    "https://api.lost-umbrella.com/user/{}/settings",
                    &user.login
                ))
                .json(&json!({
                    "name": &user.login,
                    "password": &user.password,
                }))
                .send()
                .await
                .map(|e| {
                    println!("{:#?}", &e.status());
                    e.status()
                })
                .map_err(|e| e.to_string())
        })
}
