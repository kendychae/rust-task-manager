# Module 1 (Rust) Submission Guide

## ✅ Checklist - Complete Before Submission

### 1. Code Requirements ✓

- [x] Rust project is complete and functional
- [x] Code demonstrates Rust fundamentals (ownership, borrowing, structs, enums)
- [x] Project builds successfully with `cargo build`
- [x] Project runs without errors with `cargo run`
- [x] Code is well-commented and follows Rust conventions

### 2. README.md Requirements ✓

- [x] README.md is complete with all sections filled out
- [x] Overview section explains what the software does
- [x] Development environment section lists tools and languages
- [x] Useful websites section includes learning resources
- [x] Future work section lists potential improvements
- [ ] **VIDEO LINK** - Replace `REPLACE_WITH_YOUR_VIDEO_LINK` with your actual Zoom video link

### 3. Video Requirements (4-5 minutes)

- [ ] Record video using Zoom
- [ ] Video demonstrates the software running
- [ ] Video includes a walkthrough of the code
- [ ] Video explains the Rust concepts you learned
- [ ] Upload video to university cloud storage or make unlisted on YouTube
- [ ] Get the shareable link
- [ ] Update README.md with the video link
- [ ] Include video link in Module Submit Form

### 4. GitHub Repository

- [ ] Create a new public GitHub repository named `rust-learning-project` or similar
- [ ] Push all code to the repository
- [ ] Verify README.md displays correctly on GitHub
- [ ] Test that others can clone and run your project

### 5. Module Submit Form

- [ ] Go to the Module Submit Form (link provided in assignment)
- [ ] Fill out all required fields
- [ ] Include GitHub repository link
- [ ] Include video link
- [ ] Download the Word document
- [ ] Review the checklist in the Word document

### 6. Canvas Submission

- [ ] Upload the downloaded Word document to Canvas
- [ ] Verify submission was successful

### 7. Microsoft Teams

- [ ] Share your video link in the Module 1 (Rust) Microsoft Teams channel
- [ ] View and comment on peers' videos when possible

---

## 📹 Video Content Guide (4-5 minutes)

**Introduction (30 seconds)**

- Introduce yourself
- State you're demonstrating Module 1 - Rust Learning Project
- Brief overview of what the software does

**Software Demo (1.5-2 minutes)**

- Show the program running in the terminal
- Demonstrate each feature:
  - Adding tasks
  - Listing tasks
  - Completing a task
  - Deleting a task
  - Viewing statistics
  - Exiting the program

**Code Walkthrough (2-2.5 minutes)**

- Open your code in VS Code
- Explain the main components:
  - **Task struct**: Show how it's defined and what data it holds
  - **TaskManager struct**: Explain how it manages the vector of tasks
  - **Key methods**: Highlight `add_task()`, `complete_task()`, `list_tasks()`
  - **Main function**: Show the menu loop and pattern matching
  - **Rust concepts**: Point out:
    - Ownership (`self`, `&self`, `&mut self`)
    - Borrowing and references
    - Vector usage
    - Pattern matching with `match`
    - Option types (`Option<T>`)

**Learning Reflection (30-60 seconds)**

- Briefly share what you learned about Rust
- Mention challenges you overcame
- State your confidence in using Rust for future projects

**Conclusion (15 seconds)**

- Thank viewers for watching
- Mention this is part of your software engineering portfolio

---

## 🚀 GitHub Repository Setup

If you haven't pushed to GitHub yet, follow these steps:

1. **Create a new repository on GitHub:**

   - Go to [github.com](https://github.com) and sign in
   - Click the "+" icon and select "New repository"
   - Name it: `rust-learning-project` or `rust-task-manager`
   - Make it **Public**
   - Do NOT initialize with README (you already have one)
   - Click "Create repository"

2. **Push your code:**

   ```bash
   cd rust-learning-project
   git init
   git add .
   git commit -m "Initial commit - Rust Task Manager for Module 1"
   git branch -M main
   git remote add origin https://github.com/YOUR_USERNAME/rust-learning-project.git
   git push -u origin main
   ```

3. **Verify:**
   - Visit your GitHub repository URL
   - Make sure all files are visible
   - Check that README.md displays properly

---

## 📝 Module Submit Form - Information You'll Need

Have this information ready when filling out the form:

1. **Student Information:**

   - Your name: Kendahl Bingham
   - Student ID
   - Course: CSE 310
   - Module: Module 1 - Rust Programming

2. **Project Information:**

   - Project Name: Rust Task Manager
   - GitHub Repository URL: `https://github.com/kendychae/rust-learning-project`
   - Video Link: [Your Zoom/YouTube link here]
   - Programming Language: Rust
   - Technologies Used: Rust 2021, Cargo, Standard Library

3. **Learning Objectives Met:**

   - Learned Rust ownership and borrowing system
   - Implemented structs and associated methods
   - Used vectors for dynamic data storage
   - Practiced pattern matching with match expressions
   - Handled user input and output
   - Built and tested with Cargo build system
   - Understood Rust's approach to memory safety

4. **Challenges Overcome:**

   - Understanding the borrow checker
   - Working with mutable vs immutable references
   - Using iterators and closures effectively
   - Managing String vs &str types

5. **Future Improvements:**
   - Add file persistence
   - Implement task priorities
   - Add due dates and reminders
   - Create unit tests

---

## 🎯 Key Questions You May Need to Answer

### What did you learn from this module?

"I learned the fundamentals of Rust programming, including its unique ownership system that ensures memory safety without garbage collection. I gained hands-on experience with structs, implementations, pattern matching, and working with Rust's standard library. Most importantly, I learned how Rust's compiler helps prevent common programming errors at compile time."

### Why did you choose this project?

"I chose to build a task manager because it's practical and demonstrates multiple Rust concepts in a real-world application. As a student managing coursework and family responsibilities, a task manager is personally useful while also showcasing fundamental programming patterns like CRUD operations (Create, Read, Update, Delete)."

### What challenges did you face?

"The biggest challenge was understanding Rust's ownership and borrowing rules. The compiler's error messages were initially overwhelming, but they taught me to think more carefully about data ownership and mutability. I also had to learn when to use references (&) versus owned values, and when to use mutable references (&mut)."

### What would you do differently?

"I would add more comprehensive error handling using Result types instead of unwrap(). I'd also implement unit tests from the beginning rather than as future work. Additionally, I'd explore more advanced Rust features like traits and lifetimes in future iterations."

---

## ✨ Final Tips

1. **Test everything:** Run your project one more time before submitting
2. **Proofread:** Check your README.md for typos and completeness
3. **Video quality:** Ensure your video has clear audio and visible screen
4. **Time management:** Record your video when you have uninterrupted time
5. **Be authentic:** Share your genuine learning experience in the video
6. **Backup:** Keep a copy of all files and links before submitting

**You've got this! 🚀**

Remember, this assignment demonstrates your learning journey. Be proud of what you've accomplished in learning a new programming language!
