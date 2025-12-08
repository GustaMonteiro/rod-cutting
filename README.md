# Rod Cutting Problem

This project implements different algorithmic approaches to solve the classic Rod Cutting problem for the Design and Analysis of Algorithms course in the Master's degree program.

## Problem Description

Given a rod of length `n` inches and a price list `price[]` where `price[i]` denotes the value of a piece of length `i+1`, the objective is to determine the maximum value obtainable by cutting up the rod and selling the pieces.

For example, if a rod of length 8 has the following prices for different lengths:
- Length 1: $1
- Length 2: $5  
- Length 3: $8
- Length 4: $9
- Length 5: $10
- Length 6: $17
- Length 7: $17
- Length 8: $20

The optimal solution would be to cut the rod into two pieces of lengths 2 and 6, yielding a maximum value of $5 + $17 = $22.

## Implementation Approaches

This project implements three different algorithmic approaches to solve the rod cutting problem:

### 1. Recursive Solution (`src/recursive.rs`)

**Algorithm**: A naive recursive approach that explores all possible ways to cut the rod.

**Complexity**:
- **Time**: O(2^n) - Exponential time complexity due to overlapping subproblems
- **Space**: O(n) - Recursion stack depth

**Approach**: For each possible cut position `i`, recursively compute the maximum value for the remaining rod length `n-i` and take the maximum among all possibilities.

### 2. Memoization (Top-Down DP) (`src/memoization.rs`)

**Algorithm**: Optimizes the recursive solution by storing previously computed results in a memoization table.

**Complexity**:
- **Time**: O(n²) - Each subproblem is solved only once
- **Space**: O(n) - Memoization table and recursion stack

**Approach**: Uses a `memo` vector to store computed results, avoiding redundant calculations of the same subproblems.

### 3. Dynamic Programming (Bottom-Up) (`src/dynamic_programming.rs`)

**Algorithm**: Iterative approach that builds the solution from smaller subproblems to larger ones.

**Complexity**:
- **Time**: O(n²) - Two nested loops
- **Space**: O(n) - DP table only

**Approach**: Uses a `dp` table where `dp[i]` represents the maximum value obtainable from a rod of length `i`. Builds the solution iteratively from length 1 to n.

## Project Structure

```
rod-cutting/
├── Cargo.toml              # Rust project configuration
├── README.md               # This file
└── src/
    ├── main.rs             # Main entry point and test runner
    ├── recursive.rs        # Recursive implementation
    ├── memoization.rs      # Memoization implementation
    ├── dynamic_programming.rs # Dynamic programming implementation
    └── tests.rs            # Test cases and validation
```

## Running the Code

### Prerequisites
- Rust (latest stable version)

### Build and Run
```bash
# Clone or navigate to the project directory
cd rod-cutting

# Build the project
cargo build

# Run the program (executes all three implementations with test cases)
cargo run

# Run in release mode for better performance measurement
cargo run --release
```

### Test Cases

The project includes comprehensive test cases in `src/tests.rs` that validate all implementations:

- Empty rod (edge case)
- Standard test cases from literature
- Edge cases with single-length rods
- Cases with optimal solutions at extremes

Sample test cases:
- `[1, 5, 8, 9, 10, 17, 17, 20]` → Expected: 22
- `[3, 5, 8, 9, 10, 17, 17, 20]` → Expected: 24
- `[1, 5, 8, 9]` → Expected: 10

## Performance Comparison

When running the program, you'll observe the stark difference in performance:

- **Recursive**: Becomes impractically slow for rods longer than ~20 due to exponential complexity
- **Memoization**: Efficiently handles larger inputs with O(n²) complexity
- **Dynamic Programming**: Similar performance to memoization but with better space efficiency and no recursion overhead

## Academic Context

This implementation serves as a practical study of:
- **Algorithm Design Paradigms**: Recursion, Dynamic Programming
- **Optimization Techniques**: Memoization, Tabulation
- **Complexity Analysis**: Time and space trade-offs
- **Problem-Solving Approaches**: Breaking down complex problems into subproblems

The rod cutting problem is a classic example demonstrating how Dynamic Programming can transform an exponential-time recursive solution into a polynomial-time efficient algorithm.

## License

This project is developed for academic purposes as part of the Design and Analysis of Algorithms course.