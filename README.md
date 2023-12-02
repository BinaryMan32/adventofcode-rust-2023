# adventofcode-rust-2023
Solving https://adventofcode.com/2023 in rust

Install Visual Studio Code extensions
* rust-analyzer
* Error Lens
* CodeLLDB (debugger)
* Better Toml (for `Cargo.toml`)
* crates (for `Cargo.toml`)

## Run from VS Code

Click the play button next to `main()` or tests.

To run `release` mode in VS Code with `rust-analyzer` (already configured in `.vscode`)
- Go to `Settings` > `Workspace`
- Search for `rust-analyzer.runnables.extraArgs` (default: `[]`)
- Set to `--release`

## Copy template for each day

Example:
```
cp -r src/bin/template src/bin/day01
```

## Run from terminal

Run all tests:
```
cargo test --release
```

Run tests (example input) for a single day:
```
cargo test --release --bin day01
```

Run real problem input for a single day:
```
cargo run --release --bin day01
```

Run only a single part of real problem input:
```
cargo run --release --bin day01 part2
```
