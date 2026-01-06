# Overview

As a software engineering student and single mother of two, I created this Rust Task Manager to demonstrate my understanding of Rust programming fundamentals while building practical software. This project showcases my ability to work with Rust's unique features including ownership, borrowing, structs, enums, and pattern matching.

This is an interactive command-line task manager that allows users to add, list, complete, delete tasks, and view statistics. It demonstrates core Rust concepts in a practical, user-friendly application.

The purpose of creating this software is to gain hands-on experience with Rust's memory safety features, learn the ownership system, practice working with structs and enums, and build confidence in systems programming. This project marks my first step into learning systems-level programming languages.

**Key Features:**

- ✅ Add new tasks with unique IDs
- 📋 List all tasks with completion status
- 🎉 Mark tasks as complete
- 🗑️ Delete tasks
- 📊 View task statistics (total, completed, pending)
- 💡 User-friendly command-line interface with emojis

[Software Demo Video](REPLACE_WITH_YOUR_VIDEO_LINK)

# How to Run

**Prerequisites:**

- Rust toolchain (rustc and cargo) installed. Download from [rustup.rs](https://rustup.rs/)

**Steps to run:**

1. Clone this repository:

   ```bash
   git clone https://github.com/kendychae/rust-learning-project.git
   cd rust-learning-project
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the application:
   ```bash
   cargo run
   ```

The application will start with a friendly menu interface where you can interact with your task list.

# Development Environment

**Tools Used:**

- **Visual Studio Code** - Code editor with rust-analyzer extension for intelligent code completion
- **Cargo** - Rust's package manager and build system (version 1.92.0)
- **Git** - Version control for tracking changes
- **GitHub** - Repository hosting and collaboration
- **rustc** - Rust compiler (version 1.92.0)
- **Windows PowerShell** - Terminal for running commands

**Programming Language:**

- **Rust 2021 Edition** - A systems programming language focused on safety, speed, and concurrency
- **Standard Library (`std::io`)** - For input/output operations and user interaction
- **Ownership System** - Rust's unique memory management approach without garbage collection

# Code Structure & Rust Concepts Demonstrated

This project demonstrates several key Rust programming concepts:

1. **Structs and Implementations**: `Task` and `TaskManager` structs with associated methods
2. **Ownership and Borrowing**: Proper use of references (`&self`, `&mut self`) in methods
3. **Vectors**: Dynamic arrays to store tasks (`Vec<Task>`)
4. **Pattern Matching**: Using `match` expressions for menu handling
5. **Option and Result Types**: Safe error handling with `Option<T>` and error propagation
6. **Iterators**: Using `.iter()`, `.iter_mut()`, `.filter()`, and `.find()` methods
7. **String Types**: Working with both `String` and `&str`
8. **Clone and Debug Traits**: Deriving common traits for the `Task` struct
9. **Module System**: Organizing code with proper visibility and scope

# Useful Websites

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Official comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Collection of runnable examples
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/) - Complete API reference
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Package manager documentation
- [Rust Playground](https://play.rust-lang.org/) - Online Rust code editor

# Future Work

Items to improve or add in future iterations:

- Implement file persistence to save tasks between sessions
- Add task priorities and due dates
- Implement task categories/tags
- Add search and filter functionality
- Create unit tests for core functionality
- Add error handling for edge cases
- Implement a configuration file for user preferences
