use std::{error::Error, fmt};

use card_simulation::queries::{LogIn, LogInVariables};

use super::State;

pub struct LogInHandler {
    account: Option<String>,
    password: Option<String>,
    email: Option<String>,
    should_exit: bool,
}
#[derive(Debug)]
pub enum LogInError {
    WebRequestError(String),
    GraphQLError(String),
    PasswordIncorrect,
    AccountNotFound,
}
impl fmt::Display for LogInError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogInError::WebRequestError(msg) => {
                write!(f, "WebRequestError: {}", msg)
            }
            LogInError::GraphQLError(msg) => {
                write!(f, "GraphQLError: {}", msg)
            }
            LogInError::AccountNotFound => {
                write!(f, "UserNotFound")
            }
            LogInError::PasswordIncorrect => {
                write!(f, "PasswordIncorrect")
            }
        }
    }
}
impl Error for LogInError {}
impl LogInHandler {
    pub fn new() -> Self {
        Self {
            account: None,
            password: None,
            should_exit: false,
            email: None,
        }
    }
    pub async fn run(&mut self) -> std::result::Result<(), LogInError> {
        while !self.should_exit {
            if self.account.is_none() {
                self.read_account();
            }
            match self.query_account().await {
                Ok(_) => {
                    self.read_password()?;
                    self.should_exit = true;
                }
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }
    fn read_account(&mut self) {
        println!("input your account");
        let mut account = self.account.clone().unwrap_or(String::new());
        let reader = std::io::stdin();
        reader
            .read_line(&mut account)
            .expect("failed to read account from input");
        self.account = Some(account.trim().to_string());
    }
    fn read_password(&mut self) -> std::result::Result<(), LogInError> {
        println!("input your password");
        let mut input = String::new();
        let reader = std::io::stdin();
        reader
            .read_line(&mut input)
            .expect("failed to read password from input");
        if self.password != Some(input.trim().to_string()) {
            return Err(LogInError::PasswordIncorrect);
        }
        Ok(())
    }
    async fn query_account(&mut self) -> std::result::Result<(), LogInError> {
        let client = &card_simulation::REQWEST_CLIENT;
        let resp = graphql_client::reqwest::post_graphql::<LogIn, &str>(
            client,
            card_simulation::URL.as_ref(),
            LogInVariables {
                account: self.account.clone(),
            },
        )
        .await;
        match resp {
            Ok(res) => {
                if res.errors.is_some() {
                    return Err(LogInError::GraphQLError(format!("{:?}", res.errors)));
                }
                if res.data.is_none() {
                    return Err(LogInError::AccountNotFound);
                }
                let data = res.data.unwrap();
                if data.users.len() == 0 {
                    return Err(LogInError::AccountNotFound);
                }
                self.password = data.users[0].password.clone();
                self.email = data.users[0].email.clone();
            }
            Err(err) => return Err(LogInError::WebRequestError(format!("web error: {:?}", err))),
        }
        Ok(())
    }
    pub(super) fn save_to_state(&self, state: &mut State) {
        state.account_cache.0 = self.account.clone().unwrap();
        state.account_cache.1 = self.email.clone();
        state.account_cache.2 = self.password.clone().unwrap();
    }
}
