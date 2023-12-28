# Genetic SAT

<div align="center">

![Tests](https://github.com/SamyAB/genetic-sat/actions/workflows/test.yml/badge.svg)
![Format and Lints](https://github.com/SamyAB/genetic-sat/actions/workflows/check.yml/badge.svg)

</div>

A SAT solver based on the genetic algorithm written in rust

Genetic SAT is not a production ready create. Its only purpose was to compare it with [cugen](https://github.com/SamyAB/cugen)

## Build and run solver

After cloning the repository, run:

```bash
cargo build -r
target/release/genetic_sat -p <population-size> -u <mutation-probability> -m <maximum-generation> -f /path/to/formula.dimacs
```
