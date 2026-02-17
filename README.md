# THEO-STEF-SOLVER

Solver for Inductive Logic Programming. Work in progress.

## Current state:
* Correctly parse /examples/kinship-pi/bk.pl and /examples/kinship-pi/exs.pl
* TODO: Add config file or command-line arguments so user can solve a different problem
* TODO: Write the Generate algorithm that generates candidate programs
* TODO: Write the Test algorithm that tests a candidate program on the positive and negative examples
* TODO: Write the Constrain algorithm that generates new constraints based on false positives and false negatives 

## Requirements

* `cargo` rust project manager

## Install and run

Download and run our solver:
```
git clone https://github.com/skunne/inductive_logic_programming_solver/
cd inductive_logic_programming_solver
cargo run
```
