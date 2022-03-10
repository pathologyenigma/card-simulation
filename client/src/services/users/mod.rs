mod log_in;
mod register;
pub mod queries {
    pub use super::log_in::log_in::Variables as LogInVariables;
    pub use super::log_in::LogIn;
}
pub mod mutations {
    pub use super::register::resgister::Variables as RegisterVariables;
    pub use super::register::Resgister as Register;
}
