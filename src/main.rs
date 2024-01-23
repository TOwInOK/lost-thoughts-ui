use std::fmt;

use api::comment::Comment;
use api::post::Post;
use api::role::Role;
use api::user::User;
use api::*;
use iced::keyboard::{KeyCode, Modifiers};
use iced::widget::{button, column, horizontal_space, row, scrollable, text, text_input};
use iced::{
    executor, keyboard, subscription, Application, Event, Font, Length, Subscription, Theme,
};
use iced::{Command, Settings};
use reqwest::StatusCode;

mod api;

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
    search_result: Vec<Post>,
}

#[derive(Debug, Clone)]
enum WindowState {
    Login,
    Register,
    AllPosts,
    Account,
    Poster(Post),
    Search,
}

impl fmt::Display for WindowState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WindowState::Login => write!(f, "Login"),
            WindowState::Register => write!(f, "Register"),
            WindowState::AllPosts => write!(f, "AllPosts"),
            WindowState::Account => write!(f, "Account"),
            WindowState::Poster(_) => write!(f, "Poster"),
            WindowState::Search => write!(f, "Search"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SignIn,
    Signed(Result<StatusCode, String>),
    SignUp,
    Registered(Result<StatusCode, String>),
    SwitchWindow(WindowState),
    Change(Changers),
    Find(String),
    PostAdd(Vec<Post>),
    DebugSwitch,
}
#[derive(Debug, Clone)]
enum Changers {
    PasswordChange(String),
    LoginChange(String),
    EmailChange(String),
    SearchChange(String),
}

impl Application for LostThoughts {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                user: User::new(
                    String::new(),
                    String::new(),
                    String::new(),
                    Some(Role::Default),
                ),
                logged_in: false,
                current_window: WindowState::Login,
                debbug: false,
                search: String::new(),
                title: "Login".to_string(),
                posts: vec![Post::new(
                    "Title 1".to_string(),
                    "Subtitle 1".to_string(),
                    "Body of post 1".to_string(),
                    vec!["tag1-1".to_string(), "tag1-2".to_string()],
                    vec![Comment::new(
                        "Author".to_string(),
                        "id".to_string(),
                        "reject id".to_string(),
                        "Text of shit comment".to_string(),
                    )],
                )],
                search_result: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::SignIn => Command::perform(log_in(self.user.clone()), Message::Signed),
            Message::SignUp => Command::perform(log_in(self.user.clone()), Message::Signed),
            Message::SwitchWindow(window) => {
                self.title = format!("{}", &window);
                self.current_window = window;
                Command::none()
            }
            Message::Change(changer) => {
                match changer {
                    Changers::EmailChange(value) => self.user.set_email(value),
                    Changers::LoginChange(value) => self.user.set_login(value),
                    Changers::PasswordChange(value) => self.user.set_password(value),
                    Changers::SearchChange(value) => self.search = value,
                }
                Command::none()
            }
            Message::Find(text) => {
                println!("try to find {}", text);
                Command::none()
            }
            Message::PostAdd(_) => todo!(),
            Message::DebugSwitch => {
                println!("debbug is {}", self.debbug);
                self.debbug = !self.debbug;
                Command::none()
            }
            Message::Signed(result) => match result {
                Ok(e) => match e {
                    StatusCode::OK => {
                        Command::perform(async {}, |_| Message::SwitchWindow(WindowState::AllPosts))
                    }
                    _ => Command::none(),
                },
                Err(_) => Command::none(),
            },
            Message::Registered(result) => match result {
                Ok(e) => match e {
                    StatusCode::OK => {
                        Command::perform(async {}, |_| Message::SwitchWindow(WindowState::Login))
                    }
                    _ => Command::none(),
                },
                Err(_) => Command::none(),
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match self.current_window {
            WindowState::Login => column![
                text_input("Login", self.user.get_login())
                    .on_input(|value| Message::Change(Changers::PasswordChange(value))),
                text_input("Password", self.user.get_password())
                    .password()
                    .on_input(|value| Message::Change(Changers::PasswordChange(value))),
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
                text_input("Login", self.user.get_login())
                    .on_input(|value| Message::Change(Changers::LoginChange(value))),
                text_input("Password", self.user.get_password())
                    .password()
                    .on_input(|value| Message::Change(Changers::PasswordChange(value))),
                text_input("Confirm Password", self.user.get_password()),
                text_input("Email", self.user.get_email())
                    .on_input(|value| Message::Change(Changers::EmailChange(value))),
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
            WindowState::AllPosts => column![],
            WindowState::Account => column![],
            WindowState::Poster(ref post) => column![row![
                row![button("back").on_press(Message::SwitchWindow(WindowState::Search))],
                horizontal_space(Length::Fill),
                row![column![
                    text(post.get_title()),
                    text(post.get_under_title()),
                    row![scrollable(text(&post.tag()),),]
                ]
                .align_items(iced::Alignment::Center)
                .spacing(20),],
                horizontal_space(Length::Fill),
            ],],
            WindowState::Search => {
                let search_element = column![
                    //Input field
                    row![
                        text_input("Find something?", &self.search)
                            .on_input(|value| Message::Change(Changers::SearchChange(value)))
                            .on_submit(Message::Find(self.search.clone())),
                        button("Find").on_press(Message::Find(self.search.clone())),
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                ]
                .spacing(20)
                .align_items(iced::Alignment::Center)
                .padding(30);

                //create list of result from api
                let mut result_list = column![].spacing(20);
                //parse it
                for post in self.search_result.iter() {
                    result_list = result_list.push(column![
                        button(text(post.get_title()))
                            .on_press(Message::SwitchWindow(WindowState::Poster(post.clone()))),
                        text(post.get_under_title()),
                        text(&post.tag())
                    ]);
                }
                //make it scroalbe
                let scrollable_result_list = scrollable(result_list).width(Length::Fill);
                column![search_element, scrollable_result_list].spacing(30)
            }
        };

        let logo = column![
            //Logo
            text("Monotiper").size(40),
        ]
        .padding(30)
        .align_items(iced::Alignment::Center);

        let debbug_menu = if self.debbug {
            row![
                horizontal_space(Length::Fill),
                button("Login").on_press(Message::SwitchWindow(WindowState::Login)),
                button("Register").on_press(Message::SwitchWindow(WindowState::Register)),
                button("AllPosts").on_press(Message::SwitchWindow(WindowState::AllPosts)),
                button("Account").on_press(Message::SwitchWindow(WindowState::Account)),
                button("Search").on_press(Message::SwitchWindow(WindowState::Search)),
                horizontal_space(Length::Fill),
            ]
            .align_items(iced::Alignment::Center)
            .spacing(20)
        } else {
            row![]
        };

        column![debbug_menu, logo, content]
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center)
            .into()
    }
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code,
                modifiers,
            }) if modifiers.control() => match (key_code, modifiers) {
                (KeyCode::D, Modifiers::CTRL) => Some(Message::DebugSwitch),
                _ => None,
            },
            _ => None,
        })
    }
}
