use self::role::Role;
use self::user::User;

use super::messages::{self, Changers, Message, WindowState};
use super::model::LostThoughts;
use crate::api::*;
use iced::keyboard::{KeyCode, Modifiers};
use iced::widget::{
    button, column, horizontal_space, row, scrollable, text, text_input, vertical_space,
};
use iced::Command;
use iced::{executor, keyboard, subscription, Application, Event, Length, Subscription, Theme};

///input_field!(`LABEL_TEXT`, `Value to change`, `Message to perform`)
///
///`Label` - Placeholder
///
///`Value to change` - get text to fill this field
///
///`Message to perform` - change this text in the self
macro_rules! input_field {
    ($label:expr, $value:expr, $msg:expr) => {
        text_input($label, $value).on_input(|value| Message::Change($msg(value)))
    };
}

///input_field!(`LABEL_TEXT`, `Value to change`, `Message to perform`)
///
///Automatic has `.password()`
///
///`Label` - Placeholder
///
///`Value to change` - get text to fill this field
///
///`Message to perform` - change this text in the self
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
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code,
                modifiers,
            }) if modifiers.control() => match (key_code, modifiers) {
                (KeyCode::D, Modifiers::CTRL) => {
                    Some(Message::Switcher(messages::Switch::DebugPanelSwitch))
                }
                _ => None,
            },
            _ => None,
        })
    }

    fn title(&self) -> String {
        self.title.clone()
    }

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
                posts: vec![],
                search_result: vec![],
                password: String::new(),
                password_repit: String::new(),
                prevision_screen: WindowState::None,
                forvard_screen: WindowState::None,
                label: String::new(),
                under_label: String::new(),
                text: String::new(),
                footer: String::new(),
                tags: String::new(),
                author: String::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            //Sign In
            Message::SignIn => Command::perform(log_in(self.user.clone()), Message::Signed),
            //Sign In

            //SignUp
            Message::SignUp => Command::perform(log_in(self.user.clone()), Message::Registered),
            //SignUp

            //SwitchWindow
            Message::SwitchWindow(window) => match window {
                WindowState::AllPosts => {
                    self.title = format!("{}", &window);
                    self.prevision_screen = self.current_window.clone();
                    self.current_window = window;
                    Command::perform(get_all_posts(), Message::PostAdd)
                }
                WindowState::PosterChange(ref poster) => {
                    match poster {
                        Some(e) => {
                            self.title = format!("Change post – {}", e.get_label());
                            self.label = e.get_label().to_owned();
                            self.under_label = e.get_underlabel().to_owned();
                            self.text = e.get_text().to_owned();
                            self.footer = e.get_footer().to_owned();
                            self.tags = e.get_tags_to_string();
                            self.author = e.get_author_to_string();

                            //Back & ReBack buttons state
                            self.prevision_screen = self.current_window.clone();
                            self.current_window = window;
                        }
                        None => {
                            self.title = format!("Create post");

                            //Back & ReBack buttons state
                            self.prevision_screen = self.current_window.clone();
                            self.current_window = window;
                        }
                    }
                    Command::none()
                }
                _ => {
                    self.title = format!("{}", &window);

                    //Back & ReBack buttons state
                    self.prevision_screen = self.current_window.clone();
                    self.current_window = window;
                    Command::none()
                }
            },
            //SwitchWindow

            //Change - Thing that change on input
            Message::Change(changer) => {
                match changer {
                    Changers::EmailChange(value) => self.user.set_email(value),
                    Changers::LoginChange(value) => self.user.set_login(value),
                    Changers::PasswordChange(value) => self.user.set_password(value),
                    Changers::SearchChange(value) => self.search = value,
                    Changers::Label(value) => self.label = value,
                    Changers::UnderLabel(value) => self.under_label = value,
                    Changers::Text(value) => self.text = value,
                    Changers::Footer(value) => self.footer = value,
                    Changers::Tags(value) => self.tags = value,
                    Changers::Author(value) => self.author = value,
                }
                Command::none()
            }
            //Change

            //Find
            Message::Find(text) => {
                println!("try to find {}", &text);
                Command::perform(search(text, 0), Message::PostAdd)
            }
            //Find

            //PostAdd
            Message::PostAdd(posters) => {
                println!("Start post add");
                self.posts.clear();
                match posters {
                    Ok(e) => match e {
                        Some(e) => {
                            println!("find {:#?}", e);
                            let mut e = e;
                            self.posts.append(&mut e);
                            println!("posts: {:#?}", self.posts);
                            Command::none()
                        }
                        None => Command::none(),
                    },
                    Err(e) => {
                        println!("{}", e);
                        Command::none()
                    }
                }
            }
            //PostAdd

            //Signed
            Message::Signed(result) => match result {
                Ok(e) => match e {
                    Some(e) => {
                        self.user = e;
                        Command::perform(async {}, |_| Message::SwitchWindow(WindowState::Search))
                    }
                    None => {
                        println!("SIGNED NONE");
                        Command::none()
                    }
                },
                Err(e) => {
                    println!("Signed Error: {}", e);
                    Command::none()
                }
            },
            //Signed

            //Registered
            Message::Registered(result) => match result {
                Ok(e) => match e {
                    Some(_) => {
                        Command::perform(async {}, |_| Message::SwitchWindow(WindowState::Login))
                    }
                    None => Command::none(),
                },
                Err(_) => Command::none(),
            },
            //Registered

            //Switcher
            Message::Switcher(e) => match e {
                messages::Switch::DebugPanelSwitch => {
                    println!("debbug is {}", self.debbug);
                    self.debbug = !self.debbug;
                    Command::none()
                }
                messages::Switch::ChangePasswordSwtich => todo!(),
                messages::Switch::ChangeEmailSwitch => todo!(),
            },
            //Switcher

            //Back
            Message::Back => {
                if self.prevision_screen != WindowState::None
                    && self.prevision_screen != self.current_window
                {
                    self.forvard_screen = self.current_window.clone();
                    self.current_window = self.prevision_screen.clone();
                }
                Command::none()
            }
            //Back

            //ReBack
            Message::ReBack => {
                if self.forvard_screen != WindowState::None
                    && self.forvard_screen != self.current_window
                {
                    self.current_window = self.forvard_screen.clone();
                }
                Command::none()
            }
            //ReBack
            Message::Clear => {
                self.label.clear();
                self.under_label.clear();
                self.text.clear();
                self.footer.clear();
                self.tags.clear();
                self.author.clear();
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        //Start Scrollable Poster List Sigment
        let mut result_list = column![].spacing(20).padding(30); // List
        for post in self.posts.iter() {
            //Parsing
            result_list = {
                result_list.push(column![
                    button(text(post.get_label()))
                        .on_press(Message::SwitchWindow(WindowState::Poster(post.clone()))),
                    text(post.get_underlabel()),
                    text(post.tags())
                ])
            };
        }
        //Make it scrollable
        let scrollable_result_list = scrollable(result_list).width(Length::Fill);
        //End Scrollable Poster List Sigment

        //Start Content Sigment
        let content = match &self.current_window {
            //Login Sigment
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
            //End Login Sigment

            //Start Register Sigment
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
            //End Register Sigment

            //Start AllPost Sigment
            WindowState::AllPosts => column![scrollable_result_list].spacing(30).padding(30),
            //End AllPost Sigment

            //Start Account Sigment
            WindowState::Account => {
                let bool = false;
                column![
                    //Role text
                    text(format!("Role: {}", self.user.get_role())),
                    //Account name
                    text(format!("Account name: {}", self.user.get_login())),
                    //Account email
                    text(format!("Account Email: {}", self.user.get_email())),
                    //Password rows
                    if !bool {
                        column![row![
                            text_input("SomeShit", self.user.get_password()).password(),
                            row![
                                horizontal_space(Length::Fill),
                                button("Change?"),
                                horizontal_space(Length::Fill),
                            ]
                        ]]
                    } else {
                        column![
                            //при сравнении нужно сравнить с `self.user.password`
                            text_input("You stell remember me :)!?", &self.password),
                            text_input("Write your pASSword again...", &self.password_repit),
                            //
                            row![
                                button("Cancel"),
                                horizontal_space(Length::Fill),
                                button("Push")
                            ]
                            .padding(20)
                        ]
                    },
                    //post list
                    column![//some parsed list]
                    ]
                ]
                .spacing(18)
                .padding(30)
                .align_items(iced::Alignment::Center)
                //End Account Sigment
            }
            //Start Poster Sigment
            WindowState::Poster(post) => column![row![
                row![
                    button("back").on_press(Message::SwitchWindow(WindowState::Search)),
                    button("Edit").on_press(Message::SwitchWindow(WindowState::PosterChange(
                        Some(post.clone())
                    ))),
                ],
                horizontal_space(Length::Fill),
                row![column![
                    text(post.get_label()),
                    text(post.get_underlabel()),
                    row![scrollable(text(&post.tags()),),]
                ]
                .align_items(iced::Alignment::Center)
                .spacing(20),],
                horizontal_space(Length::Fill),
            ],],
            //End Poster Sigment

            //Start PosterChange Sigment
            //change post and push it if it some, else create new post
            WindowState::PosterChange(_) => column![
                //Label
                input_field!("Lable", &self.label, Changers::Label),
                //Under Label
                input_field!("Under Lable", &self.under_label, Changers::UnderLabel),
                //Text
                input_field!("Lorem Ipsum?", &self.text, Changers::Text),
                //footer
                input_field!("Footer", &self.footer, Changers::Footer),
                //Tags
                input_field!("Milk, Cow, Grass", &self.tags, Changers::Tags),
                //Assigments (authors)
                input_field!("User1, User2, User3", &self.author, Changers::Author),
                vertical_space(Length::Fill),
                row![
                    button("clear").on_press(Message::Clear),
                    horizontal_space(Length::Fill),
                    button("push"),
                ],
            ]
            .padding(30),
            //End PosterChange Sigment

            //Start Search Sigment
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
                column![search_element, scrollable_result_list].spacing(30)
            }
            WindowState::None => column![text("404").size(90)],
        };
        //End Content Sigment

        //Start Logo
        let logo = row![
            //Logo
            button("<").on_press(Message::Back),
            button(">").on_press(Message::ReBack),
            horizontal_space(Length::Fill),
            text("Monotiper").size(40),
            horizontal_space(Length::Fill),
            if self.current_window != WindowState::Login
                && self.current_window != WindowState::Register
                && self.current_window != WindowState::Account
            {
                row![button(text(format!("Account: {}", self.user.get_login())))
                    .on_press(Message::SwitchWindow(WindowState::Account)),]
            } else {
                row![]
            }
        ]
        .padding(30);
        //End Logo

        //Start DebbugMenu
        let debbug_menu = if self.debbug {
            //do overlay
            row![
                horizontal_space(Length::Fill),
                button("Login").on_press(Message::SwitchWindow(WindowState::Login)),
                button("Register").on_press(Message::SwitchWindow(WindowState::Register)),
                button("AllPosts").on_press(Message::SwitchWindow(WindowState::AllPosts)),
                button("Account").on_press(Message::SwitchWindow(WindowState::Account)),
                button("Search").on_press(Message::SwitchWindow(WindowState::Search)),
                button("Edit Pist")
                    .on_press(Message::SwitchWindow(WindowState::PosterChange(None))),
                horizontal_space(Length::Fill),
            ]
            .align_items(iced::Alignment::Center)
            .spacing(20)
        } else {
            row![]
        };
        //End DebbugMenu

        //Output
        column![debbug_menu, logo, content]
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center)
            .into()
    }
}
