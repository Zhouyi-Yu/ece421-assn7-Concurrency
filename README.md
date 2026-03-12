# ECE 421 - Assignment 7: Concurrency

This repository contains the implementation for Assignment 7, focusing on concurrency in Rust.

## Project Structure

- **[Bank_Application](file:///Users/jookenblue/Desktop/UofA/Winter%202026/ECE_421/assn7/ece421-assn7-Concurrency/Bank_Application)**: Implementation of Question 1.
  - A thread-safe banking system using `Arc` and `Mutex` to manage account balances.
  - Simulates 10 concurrent users making transfers.
- **[Assignment7_Q2](file:///Users/jookenblue/Desktop/UofA/Winter%202026/ECE_421/assn7/ece421-assn7-Concurrency/Assignment7_Q2)**: Implementation of Question 2.
  - Fixes ownership and synchronization issues in a provided code snippet using `Arc` and `Mutex`.
  - Includes an explanation of why the original code failed in the source comments.
- **[Assignment7_Q3](file:///Users/jookenblue/Desktop/UofA/Winter%202026/ECE_421/assn7/ece421-assn7-Concurrency/Assignment7_Q3)**: Implementation of Question 3.
  - Uses the `rayon` crate to perform a parallel search for words containing a specific character.
- **[Assignment7_Q4](file:///Users/jookenblue/Desktop/UofA/Winter%202026/ECE_421/assn7/ece421-assn7-Concurrency/Assignment7_Q4)**: Implementation of Question 4.
  - A concurrent Quicksort algorithm implemented using `rayon::join` for parallel recursion.

## How to Run

Each question is a separate Cargo project. To run any of them, navigate to the directory and use `cargo run`.

```bash
cd Bank_Application
cargo run
```

## Requirements
- Rust (latest stable version)
- Cargo