# Chapter 2
## 2.1
### Managing dependencies

#### Cargo.toml

Cargo understand Semantic Versioning.

Cargo.toml:
- 0.8.5 is actually shorthand for ^0.8.5
- [caret](https://carols10cents.github.io/cargo/specifying-dependencies.html) freezes the left-most non-zero digit

```
^1.2.3 := >=1.2.3 <2.0.0
^1.2 := >=1.2.0 <2.0.0
^1 := >=1.0.0 <2.0.0
^0.2.3 := >=0.2.3 <0.3.0
^0.0.3 := >=0.0.3 <0.0.4
^0.0 := >=0.0.0 <0.1.0
^0 := >=0.0.0 <1.0.0
```

Different from package.json.

package.json:
- tilde ~ flexible patch version
- caret ^ flexible minor version

When we include external dependency, Cargo fetches the latest versions from registry.

**registry** - a copy of data from [Crates.io](https://crates.io/).

#### Cargo.lock

Creates it during the very first build: figures out all the versions, writes them down -> next time can skip the figuring out part.


Will not update dep versions automatically --> reproducible builds of an artifact.


#### Updating deps


* To update dependencies:
$ cargo update

#### Dependency docs

To run documentation provided by all of the deps:\
$ cargo doc --open


#### Ch2 Code explained

```
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"), 
        Ordering::Greater => println!("Too big!"), 
        Ordering::Equal => println!("You win!"),
    }
```
cmp(): takes a reference to whatever to compare with, returns a variant of Ordering.

A match expression is made up of arms.
An **arm** consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.

Unless otherwise specified, Rust defaults to an i32 (a 32 bit integer type).

#### Formatter

* To run in the current project:
$ cargo fmt