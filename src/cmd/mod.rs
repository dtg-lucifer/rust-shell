pub mod command;
pub mod echo;
pub mod clear;

// Re-export the main command handler function
pub use command::handle_command;
