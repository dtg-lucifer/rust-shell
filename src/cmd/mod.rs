pub mod cd;
pub mod command;
pub mod custom_exe;
pub mod echo;
pub mod pwd;
pub mod type_cmd;

// Re-export the main command handler function
pub use command::handle_command;
