# Delta Programming Language

![Delta Logo](logo/logo-banner.jpeg)

Delta is a modern, expressive programming language designed with natural language syntax that makes code more readable and intuitive. With its Python-like indentation system and English-like keywords, Delta aims to bridge the gap between human language and programming logic.

## Features

- **Natural Language Syntax**: Use phrases like "is greater than" instead of symbols
- **Indentation-based Structure**: Clean, readable code blocks without curly braces
- **Expressive Keywords**: Intuitive keywords like `let`, `be`, `when`, `then`, `otherwise`
- **Built-in Comparisons**: Natural comparison operators that read like English
- **String and Number Literals**: Full support for strings with escape sequences and floating-point numbers

## Example Code

```delta
let age be 25
let name be "Pranav"

when age is greater than or equal 18
    show "You are an adult"
otherwise
    show "You are a minor"

define greet with person_name
    show "Hello, " + person_name
end
```

## Prerequisites

### LLVM

**Linux:**
```bash
sudo apt-get update
sudo apt-get install -y build-essential gcc g++ make
sudo apt-get install -y zlib1g-dev libzstd-dev libffi-dev libncurses-dev libxml2-dev
sudo apt-get install -y llvm-17-dev libclang-17-dev clang-17 libpolly-17-dev

# Set environment variables (add to ~/.bashrc or ~/.zshrc)
export LLVM_SYS_170_PREFIX=/usr/lib/llvm-17
export LIBCLANG_PATH=/usr/lib/llvm-17/lib
export PATH=/usr/lib/llvm-17/bin:$PATH
```

**macOS:**
```bash
brew install llvm@17

# Add LLVM to your PATH (add to ~/.zshrc or ~/.bash_profile)
export LLVM_SYS_170_PREFIX=$(brew --prefix llvm@17)
export PATH="$(brew --prefix llvm@17)/bin:$PATH"
export CC=$(brew --prefix llvm@17)/bin/clang
export CXX=$(brew --prefix llvm@17)/bin/clang++
```

**Windows:**

For Windows users, we recommend using our pre-compiled LLVM builds:
- Download LLVM 17.0.6 from: https://github.com/Delta-Engine/llvm-builds-windows/releases
- Extract the archive to `C:\llvm\17.0.6` (or your preferred location)
- Add the following to your system environment variables:
  - `LLVM_SYS_170_PREFIX` = `C:\llvm\17.0.6`
  - `LIBCLANG_PATH` = `C:\llvm\17.0.6\bin`
  - Add `C:\llvm\17.0.6\bin` to your system PATH

### Rust Toolchain

- Rust compiler (rustc) - [Install from rust-lang.org](https://rust-lang.org/tools/install/)
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/PranavVerma-droid/Delta.git
cd Delta
```

2. Build the project:
```bash
cargo clean
cargo build --release
```

3. Run a Delta program:
```bash
cargo run example.de
```

### Building with Make
```bash
# Build release version
make build-release

# Build development version
make build-dev

# Test interpreter mode (requires dev build)
make test-interpret

# Test compilation to LLVM IR (requires dev build)
make test-compile
```

### Usage

Create a file with the `.de` extension and write your Delta code. Then run it using:

```bash
./target/release/delta your_file.de
```

## Language Grammar

The Delta language uses indentation to define code blocks, similar to Python. Each statement should be on its own line, and nested blocks are indicated by increased indentation.

### Basic Structure

```delta
statement
    indented_block
        nested_block
    back to previous level
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For questions, suggestions, or contributions, please open an issue on GitHub.

---

*Delta - Making programming more human* 🚀