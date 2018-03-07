#### SAFE CLI

#### Note:
This program compiles for `mock-routing` by default. Do not attempt to use it with the live network until protection of secret credentials is implemented.

##### Getting started 
- `git clone` this repository
- `cd safe_cli`
- `cargo build`
- `cargo run`
- At command prompt: type in `commands` to see list of current commands. The commands are listed in their recommended order, although try them in different order to receive helpful messages from CLI program.

##### Development dependencies
- `stable-x86_64-pc-windows-gnu` 
- `rustc v1.24.0`
- `cargo 0.25.0`

##### Goals
- Become more familiar with [safe_client_libs](https://github.com/maidsafe/safe_client_libs)
- Gain more experience and proficiency in programming with Rust
- Potentially evolve client layer for apps that don't need FFI layer from client library
- Implement Clippy to improve code and introduce perspective for how to think differently about coding in Rust
- Receive feedback from SAFE network community about what desired features
