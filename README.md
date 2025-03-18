# Rust Shell Implementation

[![Codecrafters Shell Challenge](https://img.shields.io/badge/Codecrafters-Shell%20Challenge-blue)](https://app.codecrafters.io/courses/shell/overview)

A lightweight POSIX-compliant shell written in Rust, built as part of the "Build Your Own Shell" challenge from Codecrafters.

## 📋 Features

- Command execution from PATH locations
- Built-in commands:
  - `cd` - Change directory
  - `pwd` - Print current working directory
  - `echo` - Display text
  - `type` - Display command type information
- Custom executable support
- Interactive command prompt

## 🚀 Getting Started

### Prerequisites

- Rust 1.80 or higher
- Cargo 1.82 or higher

### Installation

Clone this repository:

```bash
git clone https://github.com/dtg-lucifer/rust-shell.git
cd rust-shell
```

### Running the Shell

Build and run the shell:

```bash
cargo run
```

## 🧰 Project Structure

```
src/
├── main.rs       # Entry point
└── cmd/          # Command modules
    ├── mod.rs    # Module definitions
    ├── cd.rs     # cd builtin command
    ├── command.rs # Command handling
    ├── custom_exe.rs # Custom executable handling
    ├── echo.rs   # echo builtin command
    ├── pwd.rs    # pwd builtin command
    └── type_cmd.rs # type builtin command
```

## 🤓 How It Works

The shell provides a simple REPL (Read-Evaluate-Print Loop) interface:

1. Displays a prompt (`$`)
2. Reads user input
3. Parses the command and arguments
4. Executes the appropriate built-in command or external program
5. Returns to step 1

External commands are searched in the directories specified in the PATH environment variable.

## 🔧 Development

To contribute or modify:

1. Make your changes
2. Run tests: `cargo test`
3. Build: `cargo build`

## 📄 License

This project is available under the [MIT License](LICENSE).

## 🙏 Acknowledgements

- [Codecrafters](https://codecrafters.io) for the shell implementation challenge
