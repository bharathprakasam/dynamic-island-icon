# Development Guide

## Toolchain Setup

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install system build dependencies (e.g. Debian/Ubuntu):
   ```bash
   sudo apt install libgtk-4-dev libgtk4-layer-shell-dev build-essential
   ```

## Development Commands

- **Check syntax & types**:
  ```bash
  cargo check
  ```

- **Run unit & integration tests**:
  ```bash
  cargo test
  ```

- **Run in debug mode**:
  ```bash
  cargo run
  ```

- **Build release bundle**:
  ```bash
  cargo build --release
  ```
