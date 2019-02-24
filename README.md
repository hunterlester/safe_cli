#### SAFE CLI

#### Notes:
- This program compiles for `mock-routing` by default. Do not attempt to use it with the live network until protection of secret credentials is implemented.

##### Getting started 
- `git clone` this repository
- `cd safe_cli`
- `cargo build`
- `cargo run --bin safe_authenticatord` in one terminal
- In a separate terminal: `cargo run --bin safe_cli -- create_acc`
- Send a POST request, for example:
  - `curl -X POST http://localhost:41805/authorise/bAAAAAACTBZGGMAAAAAABGAAAAAAAAAAANB2W45DFOIXGYZLTORSXELRUHAXDGOAACYAAAAAAAAAAAR3VNFWGM33SMQQEQ5LOORSXEICMMVZXIZLSCEAAAAAAAAAAATLBNFSFGYLGMUXG4ZLUEBGHIZBOAEBAAAAAAAAAAAAHAAAAAAAAAAAF64DVMJWGSYYFAAAAAAAAAAAAAAAAAAAQAAAAAIAAAAADAAAAABAAAAAAYAAAAAAAAAAAL5YHKYTMNFRU4YLNMVZQKAAAAAAAAAAAAAAAAAABAAAAAAQAAAAAGAAAAACAAAAAAE`
  - Or, in same terminal run `cargo run --bin safe_cli -- authorise`

##### Development dependencies
- `rustc v1.32.0`

##### Goals
- Become more familiar with [safe_client_libs](https://github.com/maidsafe/safe_client_libs)
- Gain more experience and proficiency in programming with Rust
- Potentially evolve client layer for apps that don't need FFI layer from client library
- Implement Clippy to improve code and introduce perspective for how to think differently about coding in Rust
- Receive feedback from SAFE network community about desired features
- Research how specific components may have already been developed, for example Chromium's IPC management


See [DESIGN](https://github.com/hunterlester/safe_cli/blob/master/DESIGN.md) doc
