use iced::keyboard::{Modifiers, KeyCode};
use iced::subscription::{events, events_with};
use iced::widget::{button, column, horizontal_space, row, text, text_input, vertical_space, Column, scrollable};
use iced::{executor, Application, Font, Length, Theme, Element, keyboard, Subscription, subscription, event, Event};
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
    search_result: Vec<Post>
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
struct UserMin {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Post {
    title: String,
    under_title: String,
    body: String,
    tag: Vec<String>,
    comments: Vec<Comments>,
}

impl Post {
    fn tag(&self) -> String {
        self.tag.join(",")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Comments {}

#[derive(Debug, Clone)]
enum WindowState {
    Login,
    Register,
    AllPosts,
    Account,
    Poster,
    Search,
}

#[derive(Debug, Clone)]
enum Message {
    SignIn,
    Signed(Result<StatusCode, String>),
    SignUp,
    Registered(Result<StatusCode, String>),
    SwitchWindow(WindowState),
    PasswordChange(String),
    LoginChange(String),
    EmailChange(String),
    SearchChange(String),
    Find(String),
    PostAdd(Vec<Post>),
    DebugSwitch,
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
                debbug: false,
                search: String::new(),
                title: "Login".to_string(),
                posts: vec![],
                search_result: vec![]
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
            Message::SwitchWindow(window) => match window {
                WindowState::Login => {
                    self.current_window = window;
                    self.title = "Login".to_owned();
                    Command::none()
                }
                WindowState::Register => {
                    self.current_window = window;
                    self.title = "Register".to_owned();
                    Command::none()
                }
                WindowState::AllPosts => {
                    self.current_window = window;
                    self.title = "AllPosts".to_owned();
                    Command::none()
                }
                WindowState::Account => {
                    self.current_window = window;
                    self.title = "Account".to_owned();
                    Command::none()
                }
                WindowState::Poster => {
                    self.current_window = window;
                    self.title = "Poster".to_owned();
                    Command::none()
                }
                WindowState::Search => {
                    self.current_window = window;
                    self.title = "Search".to_owned();
                    Command::none()
                }
            },
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
            Message::Find(text) => {
                let mut mocked_posts = vec![
                                Post {
                                    title: "Title 1".to_string(),
                                    under_title: "Subtitle 1".to_string(),
                                    body: "Body of post 1".to_string(),
                                    tag: vec!["tag1-1".to_string(), "tag1-2".to_string()],
                                    comments: vec![Comments {}, Comments {}],
                                },
                                Post {
                                    title: "Title 2".to_string(),
                                    under_title: "Subtitle 2".to_string(),
                                    body: "Body of post 2".to_string(),
                                    tag: vec!["tag2-1".to_string(), "tag2-2".to_string()],
                                    comments: vec![Comments {}],
                                },
                                // ... Add more mocked Post instances here ...
                                Post {
                                    title: "Title 10".to_string(),
                                    under_title: "Subtitle 10".to_string(),
                                    body: "Body of post 10".to_string(),
                                    tag: vec!["tag10-1".to_string(), "tag10-2".to_string()],
                                    comments: vec![Comments {}, Comments {}, Comments {}],
                                },
                            ];
                println!("try to find {}", text);
                self.search_result.append(&mut mocked_posts);
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
            WindowState::AllPosts => column![],
            WindowState::Account => column![],
            WindowState::Poster => column![],
            WindowState::Search =>{
                let search_element = column![
                    text("Monotiper").size(40),
                    horizontal_space(30),
                    row![
                        text_input("Find something?", &self.search)
                        .on_input(Message::SearchChange)
                        .on_submit(Message::Find(self.search.clone())),
                        button("Find").on_press(Message::Find(self.search.clone())),
                    ].spacing(10).align_items(iced::Alignment::Center)
    
                ]
                .spacing(20)
                .align_items(iced::Alignment::Center)
                .padding(30);
                
                let mut result_list =  column![].spacing(20);
                for post in self.search_result.iter() {
                    result_list = result_list.push( column![
                        button(
                            text(&post.title)
                        ),
                        text(&post.under_title),
                        text(&post.tag())
                    ]);
                }
                let scrollable_result_list = scrollable(
                    result_list
                ).width(Length::Fill);
                column![
                    search_element,
                    scrollable_result_list
                ].spacing(30)
            }
        };

        let debbug_menu = if self.debbug {
            row![
                horizontal_space(Length::Fill),
                button("login").on_press(Message::SwitchWindow(WindowState::Login)),
                button("register").on_press(Message::SwitchWindow(WindowState::Register)),
                button("main").on_press(Message::SwitchWindow(WindowState::AllPosts)),
                button("account").on_press(Message::SwitchWindow(WindowState::Account)),
                button("poster").on_press(Message::SwitchWindow(WindowState::Poster)),
                button("search").on_press(Message::SwitchWindow(WindowState::Search)),
                horizontal_space(Length::Fill),
            ]
            .align_items(iced::Alignment::Center)
            .spacing(10)
        } else {
            row![]
        };

        column![debbug_menu, content, vertical_space(Length::Fill)]
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center)
            .into()
    }
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, modifiers }) if modifiers.control() => {
                    match (key_code, modifiers) {
                        (KeyCode::D, Modifiers::CTRL) => Some(Message::DebugSwitch),
                        _ => None,
                    }
                }
                _ => None,
            }
        })
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
                    println!("{:#?}", &e);
                    e.status()
                })
                .map_err(|e| e.to_string())
        })
}

async fn sign_up(user: User) -> Result<StatusCode, String> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let client = Client::new();
            client
                .post(format!("https://api.lost-umbrella.com/user/create"))
                .json(&json!({
                    "name": &user.login,
                    "password": &user.password,
                    "email": &user.email
                }))
                .send()
                .await
                .map(|e| {
                    println!("{:#?}", &e);
                    e.status()
                })
                .map_err(|e| e.to_string())
        })
}

async fn get_all_posts() -> Result<Vec<Post>, String> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let mut posts = Vec::<Post>::new();
            let client = Client::new();
            let response = client
                .get(format!("https://api.lost-umbrella.com/posts/page/all"))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let results = response
                .json::<Vec<Post>>()
                .await
                .map_err(|e| e.to_string())?;
            for result in results {
                posts.push(result);
            }
            Ok(posts)
        })
}

async fn get_page(page: u32) -> Result<Vec<Post>, String> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let mut posts = Vec::<Post>::new();
            let client = Client::new();
            let response = client
                .get(format!("https://api.lost-umbrella.com/posts/page/{}", page))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let results = response
                .json::<Vec<Post>>()
                .await
                .map_err(|e| e.to_string())?;
            for result in results {
                posts.push(result);
            }
            Ok(posts)
        })
}

async fn search(user: User) -> Result<StatusCode, String> {
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
                    println!("{:#?}", &e);
                    e.status()
                })
                .map_err(|e| e.to_string())
        })
}
