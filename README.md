# autotyping-rs

A Rust tool for automatically inferring and adding type annotations in Rust code where they are optional. This tool helps improve code readability by adding type information while maintaining Rust's type inference capabilities.

## Overview

`autotyping-rs` analyzes Rust source code and adds type annotations in places where they are optional but could improve code clarity, such as:

- Variable declarations (e.g., converting `let x = 42;` to `let x: i32 = 42;`)
- Complex expressions where type information might be helpful
- Iterator chains and closures

Note: This tool does not add types in mandatory locations (like function signatures) as those are already required by the Rust compiler.

## Installation

```bash
# Coming soon
cargo install autotyping-rs
```

## Usage

```bash
rust-autotyping <filename>
```

## Current Status

This project is currently a work in progress. Features and functionality are being actively developed.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
