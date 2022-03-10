mod services;
pub use services::users::mutations;
pub use services::users::queries;

lazy_static::lazy_static! {
    pub static ref URL: &'static str = "http://localhost:4000";
    pub static ref REQWEST_CLIENT: reqwest::Client = reqwest::Client::new();
}
