# Turing Lang

A compiled programming language implemented in Rust.

## Features

- Static typing
- LLVM-based compilation
- Integer and floating-point arithmetic
- Variables and functions
- Control flow (if/else, while)
- Advanced optimization passes

## Requirements

- Rust (latest stable version)
- LLVM 14.0
- Cargo

## Build

```bash
cargo build --release
```

## Usage

```bash
./target/release/turing_lang <source_file>
```

## Example

```
let x = 42;
let y = 10;
let result = x + y;

fn calculate(a, b) {
    return a * b;
}

if result > 50 {
    let z = calculate(result, 2);
    return z;
} else {
    return result;
}
```

## License

MIT
