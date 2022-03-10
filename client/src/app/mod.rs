mod log_in;
mod register;
use std::collections::VecDeque;

use self::{
    log_in::{LogInError, LogInHandler},
    register::{RegisterError, RegisterHandler},
};
type Result<T> = std::result::Result<T, String>;
#[async_trait::async_trait]
pub trait Application {
    // poll_events from input string
    async fn poll_events<'a>(&mut self, input: &'a str) -> Result<Event>;
    // update the state of the application
    fn update();
    // print thing to console pretend we render things
    fn render(&self);
}
#[derive(Default)]
struct Card {
    name: String,
    description: String,
}

#[derive(Default)]
struct State {
    account_cache: (String, Option<String>, String),
    draw_history: VecDeque<Card>,
}

pub struct App {
    state: State,
}

pub enum Event {
    Exit(String),
    LogIn(std::result::Result<(), LogInError>),
    Register(std::result::Result<(), RegisterError>),
}
impl Event {
    async fn parse_input_str<'a>(input: &'a str, state: &mut State) -> Result<Event> {
        match input {
            "exit" | "Exit" | "EXit" | "EXIt" | "EXIT" => Ok(Event::Exit("bye".to_owned())),
            "logIn" | "LogIn" | "log_in" | "log in" | "login" => {
                let mut handler = LogInHandler::new();
                let res = handler.run().await;
                if res.is_ok() {
                    handler.save_to_state(state)
                }
                return Ok(Self::LogIn(res));
            }
            "register" | "Register" => {
                let mut handler = RegisterHandler::new();
                let res = handler.run().await;
                if res.is_ok() {
                    handler.save_to_state(state)
                }
                return Ok(Self::Register(res));
            }
            _ => Err("unknown command, type help for more information".to_owned()),
        }
    }
}
#[async_trait::async_trait]
impl Application for App {
    async fn poll_events<'a>(&mut self, input: &'a str) -> Result<Event> {
        Event::parse_input_str(input, &mut self.state).await
    }

    fn update() {
        todo!()
    }

    fn render(&self) {
        todo!()
    }
}

impl App {
    pub fn new() -> App {
        App {
            state: State::default(),
        }
    }
}
