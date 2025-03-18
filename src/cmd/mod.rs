pub mod clear;
pub mod command;
pub mod echo;
pub mod ls;

// Re-export the main command handler function
pub use command::handle_command;
