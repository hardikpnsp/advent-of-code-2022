## Description

These are solutions for [advent of code 2022](https://adventofcode.com/2022) written in rust

### Overview
- entire repository is a [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- each binary package correspond to an exercise 
  - for example solution binary for Day 6: Tuning Trouble is the package `tuning_trouble`
- the `utils` library package contains utility method shared by all exercises
- each exercise binary package has a `/input` directory storing exercise input in text files

## Run

`cargo run --bin <name_of_the_exercies>`

for example, to run Day 6: tuning_trouble, you will fire following command  
`cargo run --bin tuning_trouble`


