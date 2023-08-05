# Chapter 1
## 1.1
### Compilation

To compile rust source file (.rs extension):
$ rustc <filename.rs>

It produces an executable binary. 
To run it:
$ ./<filename> # or .\<filename>.exe on Windows


### Formatting

Run $ rustfmt ./path/to/rust_file.rs

* Rust style is to indent with four spaces, not a tab
* Complex filenames separated by underscores
* Semicolons at the end of the line
* Exclamation mark --> it's a macro (different from a function) //explained in ch19

## 1.2
### Cargo

Compiling w/ rustc - good for small programs. For real-world projects - Cargo.

**Cargo** is Rust’s build system and package manager. 
Handles:
- building the code
- downloading the dependencies & and building those libraries. 

Comes installed with rust. Quick check:  $ cargo --version

#### Creating a project

* To create a new project:
$ cargo new <project_name>

Generates:
```
└── <project_name>
   ├── Cargo.toml
   └── src
       └── main.rs
```
TOML (Tom’s Obvious, Minimal Language) format - Cargo’s configuration format

In Rust, packages of code are referred to as crates (when referring to dependencies)


#### Building a project

* To build the project (from project directory):\
$ cargo build

It creates an executable file in <path_to_proj>/target/debug/<proj_name> (or target\debug\<proj_name>.exe on Windows).
(Puts executable inside /debug/ dir, because builds in debug mode by default)

Running $ cargo build for the first time creates Cargo.lock - a file that keeps track of exact versions of dependencies.
Automatically managed.

* To build + run the executable in one command:\
$ cargo run

* To make sure the code compiles without producing an executable (faster):\
$ cargo check

* To build for release (with optimizations):\
$ cargo build --release 
Creates an executable in target/release.
