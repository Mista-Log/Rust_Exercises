# Rust & Solana Blockchain Learning Journey

This repository documents my journey to learn Rust and build on the Solana blockchain in preparation for the AI Web3 Hackathon in March. The goal is to gain proficiency in Rust and Solana development by the end of February.

## Table of Contents
1. [Overview](#overview)
2. [Learning Plan](#learning-plan)
3. [Progress](#progress)
4. [Resources](#resources)
5. [Projects](#projects)

## Overview
I aim to:
- Learn Rust programming language fundamentals.
- Understand Solana blockchain concepts and architecture.
- Build Solana programs using Anchor.
- Deploy and interact with Solana programs through dApps.

## Learning Plan
The learning journey is structured into four weeks:

### Week 1: Rust Basics
- Day 1: Install Rust, set up the environment, and learn variables, data types, and ownership.
- Day 2: Functions, conditionals, loops, and borrowing.
- Day 3: Structs, enums, and pattern matching.
- Day 4: Collections (vectors, strings, hashmaps) and error handling.
- Day 5: Traits, generics, and reusable code.
- Day 6: Lifetimes, iterators, and closures.
- Day 7: Build a simple CLI application to reinforce concepts.

### Week 2: Solana Foundations
- Learn Solana architecture, accounts, programs, and transactions.
- Set up Solana CLI and Anchor framework.
- Build a simple on-chain program with Anchor.

### Week 3: Advanced Solana Development
- Explore Solana runtime, program-derived addresses, and Cross-Program Invocation (CPI).
- Build a program interacting with multiple accounts.

### Week 4: Building Real-World Projects
- Design and build a complete dApp (e.g., token tracker or basic DeFi protocol).
- Deploy the program and connect it to a frontend.

## Progress

### Week 1
- **Day 1:** Completed. Installed Rust and learned about variables, data types, and ownership.

#### Day 1 Details
- **Tasks Completed:**
  - Installed Rust using Rustup.
  - Set up Cargo and verified the environment.
  - Explored the Rust ownership model and how it manages memory.
  - Practiced using variables and constants.
  - Experimented with data types like integers, floats, and booleans.

- **Code Snippet:**
```rust
fn main() {
    let x = 5; // Immutable variable
    let mut y = 10; // Mutable variable
    println!("x = {} and y = {}", x, y);
    y += 5;
    println!("Updated y = {}", y);
}
```

- **Reflection:**
  Day 1 provided a solid understanding of Rust's foundation. The ownership and borrowing concepts were particularly interesting and unique compared to other programming languages.

## Resources
- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework Documentation](https://book.anchor-lang.com/)
- [Solana Discord](https://discord.com/invite/solana)

## Projects
- [Day 7 CLI Task Manager](./week1/task_manager)
- [Week 4 dApp Project](./week4/dapp)

Stay tuned for more updates as I progress through the journey!
