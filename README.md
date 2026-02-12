<p align="center">
  <h1 align="center">Rusty-Warden</h1>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-1.75+-orange?style=flat&logo=rust&logoColor=white" alt="Rust">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License">
  </a>
</p>

<p align="center">
  <i>
    Rusty-Warden is a high-performance, command-line static analysis tool designed to detect hardcoded secrets and security vulnerabilities within codebases. Built with Rust for memory safety and speed, it efficiently traverses repository structures to identify potential leaks before they reach production.
  </i>
</p>

<br>

## Features

* **Recursive Scanning:** Efficiently traverses deeply nested directory structures using `walkdir` to find secrets anywhere in your project.
* **Secret Detection:** Utilizes optimized Regular Expressions (`regex`) to identify critical patterns like AWS Keys, Private Keys, and passwords.
* **Smart Filtering:** Automatically ignores hidden files and directories (e.g., `.git`, `.env`) to reduce noise and focus on code.
* **Blazing Fast:** Leveraging `rayon` for parallel processing, Rusty-Warden scans thousands of files per second by utilizing all CPU cores.
* **Binary File Handling:** Robustly skips binary files to prevent encoding errors and false positives, ensuring reliable scans.
* **JSON Output:** Supports structured JSON output via `serde` for seamless integration with other CI/CD pipelines or reporting tools.
* **Memory Safe:** Leverages Rust's ownership model to ensure safe memory management without garbage collection pauses.

## Tech Stack

| Component | Technology | Description |
| :--- | :--- | :--- |
| **Language** | `Rust` | Systems programming language for performance and safety. |
| **CLI Argument Parsing** | `clap` | Industry-standard crate for building robust CLIs. |
| **Pattern Matching** | `regex` | High-performance regular expression engine. |
| **Serialization** | `serde` & `serde_json` | Efficient serialization framework for structured output. |
| **Error Handling** | `anyhow` | Flexible error handling for robust runtime reliability. |
| **Parallelism** | `rayon` | Data-parallelism library for multi-threaded scanning. |

## Quick Start

### Prerequisites

* [Rust & Cargo](https://www.rust-lang.org/tools/install) (Latest Stable)

### Installation

1.  **Clone the Repository**
    ```bash
    git clone https://github.com/turagik/rusty-warden.git
    cd rusty-warden
    ```

2.  **Run the Scanner**
    Execute the tool by passing the directory path you wish to scan:
    ```bash
    cargo run -- --path .
    ```

### Advanced Usage

**Scan Specific Directory:**
```bash
cargo run -- --path /path/to/project
```

**JSON Output:**
Generate machine-readable output for integration:
```bash
cargo run -- --path . --json
```

**Run Tests:**
Ensure everything is working correctly:
```bash
cargo test
```

### Expected Output

**Standard Output:**
```text
./config/app.conf:12: Found potential secret
Content: AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE

./src/secrets.rs:45: Found potential secret
Content: const PASSWORD = "super_secret_password";
```

**JSON Output:**
```json
[
  {
    "file": "./config/app.conf",
    "line": 12,
    "content": "AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE"
  }
]
```

## Project Structure

```text
rusty-warden/
├── src/
│   ├── lib.rs         # Core scanning logic and internal library
│   └── main.rs        # CLI entry point and argument parsing
├── tests/
│   └── integration_test.rs # End-to-end integration tests
├── Cargo.toml         # Dependency management
└── README.md          # Project documentation
```
