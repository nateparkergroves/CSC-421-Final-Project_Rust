
# CSC-421 Final Project
Fall 2024 

## Authors

- [@Kyan-Patel](https://github.com/Kyan-Patel)
- [@nateparkergroves](https://github.com/nateparkergroves)


## Project Overview

This repository contains code samples created for the CSC-421 Fall 2024 Final Project, showcasing key concepts in the Rust programming language. The project provides comprehensive demonstrations of Rust's unique features, with a focus on:

- Performance optimization
- Error handling mechanisms
- Concurrent programming and threading
- Memory safety principles

The code samples illustrate the powerful capabilities of Rust, highlighting its strengths in systems programming, memory management, and parallel computing.


## Directory Tree
```bash

|   README.md
|   
+---Deliverable
|   |   Cargo.lock
|   |   Cargo.toml
|   |   
|   +---src
|   |       large_text.txt
|   |       main.exe
|   |       main.pdb
|   |       main.rs   
|                           
\---Presentation_Code
    |   Cargo.lock
    |   Cargo.toml
    |   main.rs
    |   panic_error.rs
    |   python_sum.py
    |   python_vs_rust.py
    |   rust_python.rs
    |   rust_sum.rs
    |   threads_exmaple.rs
    |   
    \---Guessing_Game
            Cargo.toml
            main.rs
           
```

## Presentation Code

### Core Examples

- **`main.rs`**: Runs all Rust files in the directory to demonstrate various outputs
- **`panic_error.rs`**: Illustrates the difference between Rust compiler errors and runtime panics
- **`rust_python.rs`**: Compares Rust and Python approaches to the borrow checker
- **`rust_sum.rs`**: Demonstrates compile-time differences between native threads in Rust and imported threads in Python
- **`threads_example.rs`**: Explains the non-deterministic execution order of threads in Rust

### Additional Project

- **Guessing Game**: A Rust-based game that showcases error handling and panic mechanisms

# Word Frequency Analyzer in Rust

## Project Overview

This project demonstrates our understanding of Rust's key features, focusing specifically on the borrow checker and parallel computing. We developed a program designed to efficiently analyze large text files by counting word frequencies.

## Key Features

- **Parallel Processing**: Utilizes Rust threads to parse text concurrently, significantly improving performance
- **Memory Safety**: Implements Rust's borrow checker to ensure secure data manipulation and prevent memory leaks
- **Word Frequency Analysis**: Counts and tracks the occurrence of each word in the input text

## Getting Started

### Prerequisites

- Rust programming language installed

### Usage

1. Place your text file in the `./src/large_text.txt` file
2. Run the program to analyze word frequencies

## Technical Highlights

The program leverages Rust's unique capabilities:
- Thread-based parallel processing for faster text analysis
- Robust memory management through Rust's borrow checker
- Efficient text parsing and word counting algorithm



## Documentation

[Rust Documentation](https://www.rust-lang.org/learn)

