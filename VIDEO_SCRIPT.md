# 🎬 Minimal Video Script for Loom (4-5 minutes)

## Before Recording:
- Open terminal in project folder
- Open VS Code with main.rs
- Test Loom - ensure camera shows your face

---

## INTRO (20 seconds)
"Hi! I'm Kendahl Bingham. For CSE 310 Module 1, I built a command-line task manager in Rust to learn ownership, borrowing, and structs. Let me demo it."

## DEMO (90 seconds)
**[Terminal - type: cargo run]**

"The app loads with demo tasks. I'll show key features:

**Option 1** - Add task: 'Study Rust' - Task 4 added
**Option 2** - List tasks - Shows all with checkboxes  
**Option 3** - Complete task 1 - Now has checkmark
**Option 5** - Stats - Shows 4 total, 1 complete, 3 pending
**Option 6** - View recent tasks: shows last 2 tasks - this uses slicing
**Option 4** - Delete task 3 - Removed
**Option 7** - Exit"

## CODE WALKTHROUGH (2 minutes)
**[VS Code - main.rs]**

"Quick code overview:

**Task struct** - Has ID, description, completed status. Uses derive for Debug and Clone traits.

**TaskManager struct** - Holds a Vec of tasks. Vec is Rust's dynamic array.

**add_task method** - Uses `&mut self` - a mutable reference. This is Rust's borrowing system.

**complete_task** - Uses `iter_mut()` and `find()` returning an Option type. `if let Some()` safely handles if task exists.

**main function** - Creates mutable TaskManager. The match expression handles menu choices - it's exhaustive, compiler ensures all cases handled.

**view_recent_tasks method** - Uses slicing: `&self.tasks[start..]` creates a view into the vector without copying. This is efficient and demonstrates Rust's slice type.

**Key concepts demonstrated:**
- Variables: mutable (`mut manager`) and immutable
- Conditionals: if/else and if let
- Loops: loop and for
- Functions with references: &self and &mut self  
- Data structures: Vec
- Slicing: &self.tasks[start..] for efficient views
- Structs with impl blocks

The borrow checker was challenging but taught me to write safer code."

## CLOSING (30 seconds)
"This project taught me Rust's ownership system and memory safety. The compiler is strict but prevents bugs at compile time. I'm confident with Rust basics and ready for more advanced features. Thanks for watching!"

**[End recording]**

---

## After Recording:
- [ ] Upload to Loom
- [ ] Get shareable link  
- [ ] Add link to README.md (line 15)
- [ ] Post link in MS Teams
- [ ] Include link in submission document
