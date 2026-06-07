# Simple Calculator

A lightweight, systems-level calculator application built in Rust demonstrating core programming concepts including I/O operations, type safety, and control flow logic.

---

## 📋 Overview

This project is a command-line arithmetic calculator that performs fundamental mathematical operations through a user-friendly interactive interface. Built with Rust, it exemplifies practical application of systems programming principles including memory safety, efficient I/O handling, and type conversion mechanisms without sacrificing performance.

**Repository Created:** July 27, 2022  
**Language:** Rust (100%)

---

## ✨ Features

- **Basic Arithmetic Operations:** Addition, subtraction, multiplication, and division
- **Interactive CLI Interface:** User-friendly command-line prompts and menu-driven selection
- **Type-Safe Input Handling:** Leverages Rust's type system for robust input validation
- **Console I/O Optimization:** Proper stream flushing to ensure real-time user feedback
- **Error Resilience:** Panic-safe parsing with built-in unwrap() error handling

---

## 🛠️ Tech Stack

| Component | Details |
|-----------|---------|
| **Language** | Rust 2021 Edition |
| **Package Manager** | Cargo |
| **Standard Library** | `std::io` for input/output operations |
| **Build System** | Cargo (Rust's native build system) |

---

## 🏗️ Architecture

The application follows a **monolithic, procedural architecture** optimized for simplicity and performance:

```
User Input Layer
    ↓
Input Parsing & Type Conversion
    ↓
Control Flow (Operation Selection)
    ↓
Arithmetic Computation
    ↓
Result Output
```

### Key Architectural Decisions:

1. **Console-Based I/O:** Direct `stdin`/`stdout` interaction for minimal overhead
2. **Single-Entry Point Design:** All logic consolidated in `main()` for straightforward execution flow
3. **Sequential Processing:** Input collection, operation selection, computation, and output in linear order
4. **Type-Safe Conversion:** String-to-integer parsing with Rust's native `parse()` method

---

## 📁 Folder Structure

```
Simple-Calculator/
├── src/
│   └── main.rs              # Main application entry point
├── Cargo.toml               # Project manifest and dependencies
├── Cargo.lock               # Dependency version lock file
├── .gitignore               # Git ignore rules
└── README.md                # This file
```

---

## 🚀 Installation

### Prerequisites
- **Rust 1.56+** (Install from [https://rustup.rs/](https://rustup.rs/))
- **Cargo** (Included with Rust)

### Steps

1. **Clone the repository:**
   ```bash
   git clone https://github.com/TROJAN1HAMMER/Simple-Calculator.git
   cd Simple-Calculator
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the application:**
   ```bash
   cargo run --release
   ```

---

## 💻 Usage

Once launched, the calculator prompts for input in the following sequence:

```
Enter first number:
[User enters first operand]

Enter second number:
[User enters second operand]

[1] Add [2] Subtract [3] Multiply [4] Divide:
[User selects operation: 1, 2, 3, or 4]

The answer is [result]
```

### Example Session:

```bash
$ cargo run
Enter first number:
15
Enter second number:
3
[1] Add [2] Subtract [3] Multiply [4] Divide:
3
The answer is 45
```

---

## 🎯 Challenges Solved

### 1. **I/O Stream Management**
   - **Challenge:** Console input doesn't immediately display without buffer flushing
   - **Solution:** Implemented explicit `stdout().flush()` after each prompt to ensure real-time user feedback

### 2. **Type Safety & Conversion**
   - **Challenge:** User input arrives as strings; mathematical operations require numeric types
   - **Solution:** Leveraged Rust's `parse()` method with type inference and `trim()` to handle whitespace, ensuring type-safe conversions

### 3. **Arithmetic Operation Selection**
   - **Challenge:** Mapping user menu selection to appropriate operations
   - **Solution:** Implemented conditional branching (`if-else`) structure with integer pattern matching for clean operation dispatch

### 4. **Error Resilience**
   - **Challenge:** Invalid input could crash the application
   - **Solution:** Used Rust's `unwrap()` for straightforward error propagation (production code would use `Result` types for graceful error handling)

---

## 🔮 Future Improvements

1. **Advanced Error Handling**
   - Replace `unwrap()` with `Result<T, E>` types for graceful error recovery
   - Implement custom error types for domain-specific error messages

2. **Extended Operations**
   - Add support for modulo (`%`), exponentiation (`^`), and square root operations
   - Implement trigonometric and logarithmic functions

3. **Persistent History**
   - Store calculation history to a local file
   - Allow users to recall and repeat previous calculations

4. **Floating-Point Support**
   - Extend from `i32` to `f64` for decimal calculations
   - Implement precision rounding mechanisms

5. **GUI/Web Interface**
   - Develop a web-based frontend using frameworks like **Yew** or **Tauri**
   - Create cross-platform desktop application

6. **Unit Testing**
   - Implement comprehensive test suites using Rust's built-in `#[cfg(test)]` module
   - Achieve >80% code coverage with edge case validation

7. **Documentation**
   - Generate API documentation using `cargo doc`
   - Add inline code comments for complex logic

8. **Performance Optimization**
   - Benchmark against competing implementations
   - Profile memory usage patterns

---

## 👥 Authors

- **[TROJAN1HAMMER](https://github.com/TROJAN1HAMMER)** - Original Developer
- **HARSHITH B** - Contributor

---

## 📄 License

This project is open-source and available for educational and professional use.

---

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Fork this repository
- Create a feature branch (`git checkout -b feature/amazing-feature`)
- Commit your changes (`git commit -m 'Add amazing feature'`)
- Push to the branch (`git push origin feature/amazing-feature`)
- Open a Pull Request

---

## 📞 Support

For questions, issues, or suggestions, please open an [issue](https://github.com/TROJAN1HAMMER/Simple-Calculator/issues) on GitHub.

---

**Built with ❤️ in Rust**
