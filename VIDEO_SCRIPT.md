# 🎬 Video Recording Script (4-5 minutes)

Use this script as a guide when recording your Zoom video. Practice once or twice before the final recording.

---

## Setup Before Recording:

- [ ] Close unnecessary applications
- [ ] Clear your terminal
- [ ] Have VS Code open with your project
- [ ] Test your microphone
- [ ] Position your screen comfortably
- [ ] Have a glass of water nearby

---

## 🎥 SCRIPT

### INTRO (30 seconds)

**[Show your face or start with screen sharing]**

"Hi everyone! My name is Kendahl Bingham, and I'm a software engineering student at BYU-Idaho. Today I'm presenting my Module 1 project for CSE 310, where I learned Rust programming.

For this module, I built a command-line task manager application in Rust to demonstrate fundamental concepts like ownership, borrowing, structs, and pattern matching. Let me show you how it works."

---

### SOFTWARE DEMO (1.5-2 minutes)

**[Switch to terminal/command prompt, navigate to project folder]**

"I'm going to run the application using Cargo, which is Rust's build tool and package manager."

**[Type: cargo run]**

"As you can see, the application starts with a welcome message and some demo tasks already loaded. Let me walk through the features.

**[Show the menu]**

The menu gives us six options. Let me demonstrate each one:

**1. Adding a Task:**
**[Select option 1]**
"First, I'll add a new task."
**[Type: "Study Rust ownership and borrowing"]**
"Great! Task 4 has been added successfully.

**2. Listing Tasks:**
**[Select option 2]**
"Option 2 lists all our tasks. You can see the demo tasks I created, plus the new one I just added. Notice the checkbox shows which tasks are complete.

**3. Completing a Task:**
**[Select option 3]**
"Now I'll mark task 1 as complete."
**[Type: 1]**
"Perfect! Task 1 is now marked as complete. If I list the tasks again..."
**[Select option 2]**
"...you can see task 1 now has a checkmark.

**4. Viewing Statistics:**
**[Select option 5]**
"The statistics feature shows us a summary: we have 4 total tasks, 1 completed, and 3 still pending.

**5. Deleting a Task:**
**[Select option 4]**
"Let me delete task 3."
**[Type: 3]**
"Task 3 has been deleted."

**6. Exiting:**
**[Select option 6]**
"And finally, option 6 exits the program with a friendly message.

So that's the complete functionality of the task manager."

---

### CODE WALKTHROUGH (2-2.5 minutes)

**[Switch to VS Code with main.rs open]**

"Now let me walk you through the code and explain the Rust concepts I learned.

**[Scroll to top of file]**

"At the top, I'm importing the standard library's IO module for handling user input and output.

**[Show Task struct]**

"Here's my Task struct. In Rust, structs are like classes in other languages. Each task has an ID, a description, and a completion status. I'm using the derive attribute to automatically implement Debug and Clone traits, which give me useful functionality.

**[Show Task implementation]**

"This `impl` block defines methods for the Task struct. The `new` function is a constructor that creates a new task. Notice I'm using `Self` which refers to the Task type. The `display` method shows how to format output.

**[Show TaskManager struct]**

"The TaskManager struct holds a vector of tasks - that's Rust's dynamic array type - and tracks the next ID to assign.

**[Show add_task method]**

"This is where Rust's ownership system comes in. Notice the method signature uses `&mut self` - this is a mutable reference. In Rust, you can have either one mutable reference OR multiple immutable references, but not both. This prevents data races at compile time.

When I push a task to the vector, ownership of that task is transferred to the vector.

**[Show complete_task method]**

"This method demonstrates borrowing and the Option type. The `iter_mut()` gives us mutable references to each task. The `find()` method returns an Option - either Some with the task we found, or None if it doesn't exist. The `if let Some()` pattern matches on that Option safely.

**[Show list_tasks method]**

"Here I'm using an immutable reference `&self` because I'm just reading the tasks, not modifying them. The for loop borrows each task with `&self.tasks`.

**[Show main function]**

"In the main function, I create a mutable TaskManager and add some demo tasks. The `.to_string()` converts string literals to owned String types.

**[Show match expression]**

"This match expression is Rust's version of a switch statement, but it's much more powerful. It's exhaustive - the compiler ensures I handle all possible cases. I'm matching on string slices here and handling each menu option.

The error handling uses Rust's Result type - `parse()` returns a Result that's either Ok with the parsed number or Err if parsing failed.

**[Scroll back up]**

"These are the key Rust concepts I learned:

1. Ownership - data has exactly one owner
2. Borrowing - you can borrow data with references
3. Mutability - clearly marked with the `mut` keyword
4. Pattern matching - for safe, exhaustive handling of cases
5. Type safety - the compiler prevents many common bugs

The compiler was strict, but that strictness taught me to write safer code."

---

### REFLECTION (30-60 seconds)

**[Can show your face or stay on screen]**

"Working on this project taught me a lot about systems programming. The biggest challenge was understanding the borrow checker - initially, those compiler errors were frustrating, but I learned they were actually helping me avoid bugs.

What I appreciate most about Rust is how it makes me think carefully about data ownership and mutability. These are concerns in every language, but Rust makes them explicit and enforced at compile time.

I feel confident building basic Rust applications now, and I'm excited to explore more advanced features like traits, lifetimes, and concurrent programming in future projects.

This project showed me that stepping out of my comfort zone to learn challenging technologies is both possible and rewarding."

---

### CLOSING (15 seconds)

"Thanks for watching my Module 1 presentation! This Rust Task Manager is now part of my software engineering portfolio on GitHub. If you have questions or feedback, feel free to reach out. Happy coding!"

**[End recording]**

---

## Post-Recording Checklist:

- [ ] Review the video for audio/visual quality
- [ ] Upload to Zoom cloud or YouTube (unlisted)
- [ ] Get the shareable link
- [ ] Update README.md with the link
- [ ] Include link in submission form

---

## Backup Topics (if you finish early):

- Mention how Cargo.toml manages dependencies
- Talk about how Rust prevents null pointer exceptions
- Discuss why Rust is becoming popular for web backends and systems programming
- Share your favorite Rust learning resource

## If Running Over Time:

- Shorten the demo section (show 3-4 features instead of all 6)
- Reduce code walkthrough to just the Task struct and one method
- Keep reflection brief (20-30 seconds)

**You've got this! Be confident and enthusiastic! 🎬🦀**
