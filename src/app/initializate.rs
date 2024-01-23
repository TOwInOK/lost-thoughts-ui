use self::comment::Comment;
use self::post::Post;
use self::role::Role;
use self::user::User;

use super::messages::{Changers, Message, WindowState};
use super::model::LostThoughts;
use crate::api::*;
use iced::keyboard::{KeyCode, Modifiers};
use iced::widget::{button, column, horizontal_space, row, scrollable, text, text_input};
use iced::Command;
use iced::{executor, keyboard, subscription, Application, Event, Length, Subscription, Theme};
use reqwest::StatusCode;

macro_rules! input_field {
    ($label:expr, $value:expr, $msg:expr) => {
        text_input($label, $value).on_input(|value| Message::Change($msg(value)))
    };
}
macro_rules! secure_input_field {
    ($label:expr, $value:expr, $msg:expr) => {
        text_input($label, $value)
            .on_input(|value| Message::Change($msg(value)))
            .password()
    };
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
            Message::SignUp => Command::perform(log_in(self.user.clone()), Message::Registered),
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
                input_field!("Login", self.user.get_login(), Changers::LoginChange),
                secure_input_field!(
                    "Password",
                    self.user.get_password(),
                    Changers::PasswordChange
                ),
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
                input_field!("Login", self.user.get_login(), Changers::LoginChange),
                secure_input_field!(
                    "Password",
                    self.user.get_password(),
                    Changers::PasswordChange
                ),
                text_input("Confirm Password", self.user.get_password()),
                input_field!("Email", self.user.get_email(), Changers::EmailChange),
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
                        input_field!("Find something?", &self.search, Changers::SearchChange)
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
