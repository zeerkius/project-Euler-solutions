# Project Euler Solutions (Rust)

A growing collection of **Project Euler** problem solutions written in **Rust**, designed to explore mathematics, algorithms, and computational problem solving through efficient and reusable implementations. This repository serves both as a solution archive and as a practical environment for experimenting with mathematical programming, optimization techniques, and algorithmic reasoning.

Project Euler problems emphasize **mathematical insight combined with efficient computation**, making them an excellent bridge between theoretical mathematics and real-world computer science.
- **proofs in discussion :) :)** 
---

## Motivation

Project Euler is more than a programming challenge set — it represents a computational exploration of topics across:

- Number Theory
- Combinatorics
- Probability & Statistics
- Optimization
- Dynamic Programming
- Graph Theory
- Simulation & Monte Carlo Methods
- Numerical Methods
- Algorithm Design & Complexity Analysis

Many problems cannot be solved efficiently through brute force alone and instead require recognizing mathematical structure, symmetry, or probabilistic behavior.

This repository focuses on implementing solutions that highlight both:

✅ mathematical reasoning  
✅ performant systems-level implementation in Rust

---

## Mathematical & Computational Topics Covered

### Number Theory
A large portion of Project Euler revolves around properties of integers:

- Prime generation and sieves
- Modular arithmetic
- Factorization
- Euler’s Totient function
- Diophantine equations
- Digit and permutation analysis

Example:
- Circular primes
- Prime sums
- Highly composite numbers

---

### Combinatorics
Problems frequently involve counting structured arrangements:

- Permutations and combinations
- Integer partitions
- Counting paths
- Inclusion–exclusion principles
- Generating functions

These problems often transition naturally into **dynamic programming** solutions.

---

### Probability & Statistics
Many Euler problems model stochastic systems:

- Expected value computation
- Random processes
- Markov-style transitions
- Distribution analysis
- Conditional probability

Solutions sometimes require analytical derivations before implementation.

---

### Monte Carlo & Simulation
Some problems become tractable through simulation:

- Random sampling
- Empirical estimation
- Convergence experiments
- Approximation of probabilistic outcomes

Rust enables fast simulation loops suitable for large experimental runs.

---

### Optimization & Algorithm Design
A recurring theme in Project Euler is transforming exponential searches into efficient algorithms:

- Search space pruning
- Memoization
- Branch-and-bound reasoning
- Dynamic programming
- Mathematical reduction of complexity

Many problems move from naïve \(O(n^k)\) solutions toward near-linear or logarithmic performance.

---

### Graph Theory & Discrete Structures
Certain problems map naturally to graph-based formulations:

- State-space exploration
- Path optimization
- Constraint satisfaction
- Coloring and connectivity problems

These connections link Euler problems to classical CS topics such as CSPs and combinatorial optimization.

---

## Repository Design

Solutions are exposed through a unified interface:

```rust
mod internal;
use internal::Problems;

fn main() {
    let p = Problems;

    println!("Number of Circular Primes {:?}", p.p35(1_000_000));
}