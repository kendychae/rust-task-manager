# Module 1: Rust Video Guide

## ✅ BEFORE YOU START RECORDING

**Open these windows:**

1. PowerShell/Terminal in your project folder: `c:\Users\kendy\Desktop\rust-task-manager`
2. VS Code with main.rs open
3. Loom recording software

**Test your setup:**

- Turn on your webcam - make sure your face is visible
- Test audio - speak and listen to playback
- Position windows so you can share your screen clearly

**Quick test run:**

- Type `cargo run` in terminal to make sure program works
- Close the program (option 7)
- Clear terminal: `cls`

---

## 🎬 PART 1: INTRODUCTION (20-30 seconds)

### What to do:

Start Loom recording. Make sure your webcam shows your face in a small window.

### What to say (read this exactly):

> "Hi! My name is Kendahl Bingham, and I'm a software developer.
>
> I built this command-line task manager in Rust to demonstrate my understanding of systems programming.
>
> This project showcases variables, conditionals, loops, functions, data structures, and Rust's unique ownership system.

---

## 🎬 PART 2: SOFTWARE DEMO (1.5 minutes)

### What to do:

Share your screen showing the terminal.

### What to say and type:

**Say:** "I'll run the program using Cargo, which is Rust's build tool."

**Type in terminal:** `cargo run`

**Say:** "The program starts and shows a menu with options. Let me demonstrate each feature."

---

### Demo Feature 1: Add a Task

**Say:** "First, I'll add a new task."

**Type:** `1` (press Enter)

**Say:** "I'll add a task to study Rust."

**Type:** `Study Rust ownership`

**Say:** "Task 4 has been added successfully."

---

### Demo Feature 2: List All Tasks

**Say:** "Now let's list all tasks."

**Type:** `2`

**Say:** "You can see all four tasks. Notice the checkboxes show which tasks are complete and which are pending."

---

### Demo Feature 3: Complete a Task

**Say:** "Let me mark task 1 as complete."

**Type:** `3`

**Type:** `1`

**Say:** "Task 1 is now complete. Let me list the tasks again to show the change."

**Type:** `2`

**Say:** "See? Task 1 now has a checkmark."

---

### Demo Feature 4: View Statistics

**Say:** "The program can show statistics."

**Type:** `5`

**Say:** "This shows we have 4 total tasks, 1 completed, and 3 pending."

---

### Demo Feature 5: View Recent Tasks (Slicing)

**Say:** "This feature uses slicing to show recent tasks efficiently."

**Type:** `6`

**Type:** `2`

**Say:** "This shows the last 2 tasks without copying all the data."

---

### Demo Feature 6: Delete a Task

**Say:** "Let me delete task 3."

**Type:** `4`

**Type:** `3`

**Say:** "Task 3 has been deleted."

---

### Demo Feature 7: Exit

**Say:** "Finally, I'll exit the program."

**Type:** `7`

**Say:** "That's all the features. Now let me show you the code."

---

## 🎬 PART 3: CODE WALKTHROUGH (2 minutes)

### What to do:

Switch to VS Code showing main.rs

---

### Show: Task Struct (lines 7-11)

**Say:** "Let me explain the code and the Rust concepts I learned.

First is the Task struct. A struct is like a container that holds related data. Each task has an ID number, a description text, and a completed status that's true or false."

---

### Show: impl Task (lines 13-25)

**Say:** "This impl block adds methods to the Task struct.

The 'new' function creates a new task.

The 'display' method prints the task with a checkbox."

---

### Show: TaskManager Struct (lines 27-30)

**Say:** "The TaskManager struct holds a Vec of tasks. Vec is Rust's dynamic array - it can grow and shrink.

This demonstrates using data structures."

---

### Show: add_task Method (lines 42-47)

**Say:** "This add_task method demonstrates Rust's ownership system.

See the ampersand mut self? That's a mutable reference. It means we're borrowing the TaskManager to change it, but we don't own it.

This is how Rust prevents bugs - you can't have two parts of code changing the same data at the same time."

---

### Show: complete_task Method (lines 62-68)

**Say:** "This complete_task method shows more Rust features.

The 'iter mut' creates a mutable iterator - it lets us look through tasks and change them.

The 'find' method returns an Option - it's either Some with the task we found, or None if it doesn't exist.

The 'if let Some' safely handles both cases. This prevents null pointer errors."

---

### Show: view_recent_tasks Method (lines 87-103)

**Say:** "This method demonstrates slicing.

On line 94, the expression 'self dot tasks square bracket start dot dot' creates a slice.

A slice is a view into the data without copying it. This is efficient and safe."

---

### Show: main Function (lines 125-end)

**Say:** "The main function brings it all together.

I create a mutable TaskManager using 'let mut'. The 'mut' keyword means it can be changed.

The loop runs forever until we break out.

The match expression handles user input. Match is like a switch statement, but better - the compiler makes sure I handle every possible case."

---

### Show: Overall Concepts

**Say:** "This project demonstrates all the required Rust concepts:

Variables - both mutable with 'mut' and immutable.

Expressions - like arithmetic and boolean logic.

Conditionals - if statements and match expressions.

Loops - the main loop and for loops.

Functions - with references like ampersand self and ampersand mut self.

Data structures - the Vec to store tasks.

Slicing - viewing data efficiently.

And structs with impl blocks for object-oriented programming."

---

## 🎬 PART 4: REFLECTION & CLOSING (30 seconds)

### What to do:

You can show your face again or stay on code screen.

### What to say:

> "The hardest part was understanding Rust's borrow checker. At first, the compiler errors were confusing. But I learned they're actually helpful - they prevent bugs before the program even runs.
>
> This project taught me to think carefully about who owns data and how it's used. That's valuable in any programming language.
>
> I'm now confident with Rust basics and excited to learn more advanced features.
>
> Thanks for watching!"

**Stop recording.**

---

## ✅ AFTER RECORDING

1. **Watch your video** - Make sure:

   - Your face is visible
   - Audio is clear
   - All features were demonstrated
   - Code was explained

2. **Upload to Loom** - Get the shareable link

3. **Update README.md** - Add video link on line 15

4. **Post to MS Teams** - Share in the Module 1 channel

5. **Submit Assignment** - Include video link in submission document

---

## 📝 TIPS FOR SUCCESS

- **Speak slowly and clearly** - Pretend you're teaching someone who knows nothing about Rust
- **Don't worry about being perfect** - Authenticity is more important
- **Smile and be enthusiastic** - Your energy makes the video engaging
- **If you mess up** - Just pause, take a breath, and continue. You can edit later.
- **Time yourself** - Practice once to make sure you're around 4-5 minutes

**You've got this! 🎉**
