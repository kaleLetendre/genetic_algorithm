# 🧬 Genetic Algorithm - String Matching in Rust

This goal of this project build a simple genetic algorithm in Rust over a 48 hour period to familiarize myself with the language.
the algorithm is meant to evolve a population of binary-encoded individuals to match a target ASCII string (e.g., `"hello_world"`). The algorithm uses crossover, mutation, and a fitness function based on string similarity to iteratively improve the population toward an ideal solution.

---

## 📁 Project Structure

```
.
├── src
│   ├── main.rs              # Entry point demonstrating string matching
│   └── genetic_algorithm.rs # Implementation of the genetic algorithm
```

---

## 🚀 Features

- ASCII string matching using genetic algorithms.
- Bitwise and bytewise crossover strategies.
- Multi-threaded population evolution using `std::thread` and `num_cpus`.
- Fitness function based on ASCII character similarity.
- Individuals represented as binary gene vectors.
- Configurable mutation rate and population parameters.

---

## 🔧 How It Works

### String Matching

The algorithm attempts to evolve a population to match a given ASCII string (e.g., `"hello_world"`). Each individual is a sequence of `bool` genes, grouped in 8s to represent ASCII characters.

**Fitness Calculation:**

```rust
fitness += 255 - abs(individual_byte - target_byte);
```

Higher fitness = closer ASCII match.

### Genetic Operations

- **Selection:** Top `N` individuals are retained.
- **Crossover:** Each child's genes are built from random slices of parent genes (bitwise or bytewise).
- **Mutation:** Each gene has a configurable probability of flipping.

### Termination

The loop stops when:
- A perfect match is found (fitness = `255 * len(string)`), or
- You manually terminate (e.g., with `Ctrl+C`).

---

## 🛠️ Usage

### 🧪 Run the Example

Make sure you have Rust installed, then run:

```bash
cargo run
```

This will evolve a population to match the string `"hello_world"`.

---

## ⚙️ Configuration

You can modify the following parameters in `main.rs`:

```rust
let goal_fitness = 12 * 255; // perfect match for "hello_world"
let gene_length = string.len() * 8;
let mut population = genetic_algorithm::init_population(
    gene_length,       // gene length (bytes * 8)
    10,                // population size
    4,                 // number of parents
    15,                // mutation rate (%)
    true,              // multi-threaded
    CrossoverType::Byte
);
```

---

## 📦 Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
num_cpus = "1.13"
```

---
