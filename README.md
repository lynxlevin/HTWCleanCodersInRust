# HTWCleanCodersInRust
HTWCleanCodersInRust is a Rust implementation of [Uncle Bob's Hunt the Wumpus in clean architecture](https://github.com/unclebob/HTWCleanCoders).
I built this project,
- to learn Rust Programming Language,
- to practice clean architecture
- and to experience converting an existing application to another language.

# Installation
1. Install Rust, following [the official guide](https://www.rust-lang.org/learn/get-started).
2. Run `cargo run` in htw directory.
3. Game begins.

# Usage
The rule is to wander round in a cavern and hunt the wumpus.
You can move around by `n`(North), `e`(East), `s`(South), `w`(West) commands.
You can shoot an arrow by `sn`(Shoot North), `se`(Shoot East), `ss`(Shoot South), `sw`(Shoot West) commands.
When your arrow hits the wumpus, you win. If the wumpus finds you, you lose.
