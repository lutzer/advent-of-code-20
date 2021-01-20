# Advent of Code 2020

Problems solved in rust.

## Installation

* install rustup and cargo: https://rustup.rs/:
  * `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  * install cargo-edit `cargo install cargo-edit`, it allows to add dependencies with `cargo add <package>`
  * create new project with `cargo new <project_name>`

## Build Instructions

### build in cli
  ```
  > cargo build
  > cargo run -- <part>
  ```
### build using visual studio task:
  * create `.vscode/tasks.json`
  ```
  {
    "version": "2.0.0",
    "tasks": [{
     "label": "cargo build",
     "type": "shell",
     "command": "cargo build",
     "args": [],
     "group": {
       "kind": "build",
       "isDefault": true
     }
    },
    {
        "label": "cargo run",
        "type": "shell",
        "command": "cargo",
        "args": [
          "run"
          // "--release",
          // "--",
          // "arg1"
        ],
        "group": {
          "kind": "build",
          "isDefault": true
        }
       }]
  }
  ```