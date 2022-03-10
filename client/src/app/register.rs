use std::error::Error;
use std::fmt;

use card_simulation::URL;


use super::super::services::users::mutations::{Register, RegisterVariables};
use super::State;
#[derive(Clone)]
pub enum VariableNames {
    Username,
    Password,
    ConfirmPassword,
    Email,
}
impl VariableNames {
    fn iter(&self) -> VariableNamesIter {
        VariableNamesIter(Some(self.clone()))
    }
}
/// iterator implementation for variables
/// to make it available to call .next()
/// to reach the next variable when done parsing one variable
struct VariableNamesIter(Option<VariableNames>);
impl Iterator for VariableNamesIter {
    type Item = (bool ,VariableNames);

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.0.clone();
        if let Some(inner) = self.0.clone() {
            match inner {
                VariableNames::Username => self.0 = Some(VariableNames::Password),
                VariableNames::Password => self.0 = Some(VariableNames::ConfirmPassword),
                VariableNames::ConfirmPassword => self.0 = Some(VariableNames::Email),
                VariableNames::Email => self.0 = None,
            }
        }
        if res.is_none() {
            None
        } else {
            Some((self.0.is_none(), res.unwrap()))
        }
        
    }
}
/// for printing like this:
/// input your XXX
impl fmt::Display for VariableNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let common_str = "input your ";
        match self {
            VariableNames::Username => {
                write!(f, "{}username", common_str)
            }
            VariableNames::Password => {
                write!(f, "{}password", common_str)
            }
            VariableNames::ConfirmPassword => {
                write!(f, "{}confirm password", common_str)
            }
            VariableNames::Email => {
                write!(f, "{}email", common_str)
            }
        }
    }
}
/// this handlder don't need should_exit
/// because the read_input should block
pub struct RegisterHandler {
    /// because graphql api will ignore the null input value
    /// so we can use the variables type directly
    variables: RegisterVariables,
    /// don't want to write code like:
    /// read_username ...
    /// read_password ...
    /// read_confirm_password ...
    /// read_email ...
    /// so we could making it to be read at same function
    /// and could be changed by just changed the enum
    current_reading: VariableNames,
}
#[derive(Debug)]
pub enum RegisterError {
    WebRequestError(String),
    /// because we set up unique index for the username and email
    /// so the unique error will be inside this type of error
    /// map through this error will help us to know if the username or email is taken
    GraphQLError(bool, String),
}
impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegisterError::WebRequestError(msg) => {
                write!(f, "WebRequestError: {}", msg)
            }
            RegisterError::GraphQLError(is_server_error, msg) => {
                if *is_server_error {
                    write!(f, "Internal Server Error: {}", msg)
                } else {
                    write!(f, "{}", msg)
                }
            }
        }
    }
}
impl Error for RegisterError {}

impl RegisterHandler {
    pub fn new() -> Self {
        Self {
            variables: RegisterVariables {
                username: None,
                email: None,
                password: None,
            },
            current_reading: VariableNames::Username,
        }
    }
    pub async fn run(&mut self) -> Result<(), RegisterError> {
        let read_input_res = self.read_input();
        if read_input_res.is_err() {
            return read_input_res;
        }
        let send_insert_request_res = self.send_insert_request().await;
        if send_insert_request_res.is_err() {
            return send_insert_request_res;
        }

        Ok(())
    }
    fn read_input(&mut self) -> Result<(), RegisterError> {
        let (mut input, reader) = (String::new(), std::io::stdin());
        let mut iter = self.current_reading.iter();
        while let Some((is_optional,variable_name)) = iter.next() {
            println!("{}", variable_name);
            reader.read_line(&mut input).expect("failed to read input");
            while input.trim().is_empty() && !is_optional {
                eprintln!("empty input not allowed");
                reader.read_line(&mut input).expect("failed to read input");
            }
            let input_str = input.trim();
            match variable_name {
                VariableNames::Username => {
                    self.variables.username = Some(input_str.to_owned());
                }
                VariableNames::Password => {
                    self.variables.password = Some(input_str.to_owned());
                }
                VariableNames::ConfirmPassword => {
                    if self.variables.password != Some(input_str.to_owned()) {
                        eprintln!("password not matched");
                        iter = variable_name.iter();
                    }
                }
                VariableNames::Email => {
                    let email = if input_str.is_empty() {
                        None
                    } else {
                        Some(input_str.to_owned())
                    };
                    self.variables.email = email;
                }
            }
            input = String::new();
        }
        Ok(())
    }
    async fn send_insert_request(&mut self) -> Result<(), RegisterError> {
        let client = &card_simulation::REQWEST_CLIENT;
        let resp = graphql_client::reqwest::post_graphql::<Register, &str>(client, URL.as_ref(), RegisterVariables{
            username: self.variables.username.clone(),
            password: self.variables.password.clone(),
            email: self.variables.email.clone(),
        }).await;
        match resp {
            Ok(res) => {
                if res.errors.is_some() {
                    let errors = res.errors.unwrap();
                    if errors[0].message.contains("Constraint") {
                        return Err(RegisterError::GraphQLError(false, "username or email is taken".to_owned()));
                    }
                    return Err(RegisterError::GraphQLError(true, format!("{:?}", errors)));
                }
            },
            Err(err) => {
                return Err(RegisterError::WebRequestError(format!("{:?}", err)));
            },
        }
        Ok(())
    }
    /// this function exists
    /// because we may need to automatically login
    /// when register is successful
    pub(super) fn save_to_state(&mut self, state: &mut State) {
        state.account_cache.0 = self.variables.username.clone().unwrap();
        state.account_cache.1 = self.variables.email.clone();
        state.account_cache.2 = self.variables.password.clone().unwrap();
    }
}
