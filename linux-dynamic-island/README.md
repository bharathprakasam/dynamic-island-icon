# Linux Dynamic Island

A modern, highly customizable, and fluid dynamic island overlay widget for Linux desktop environments, supporting both Wayland and X11.

## Overview

Linux Dynamic Island brings interactive status indicators, quick application launchers, notifications, and media controls right to the top of your desktop in an elegant expanding capsule.

## Directory Structure

```
linux-dynamic-island/
├── Cargo.toml
├── Cargo.lock
├── build.rs
├── README.md
├── LICENSE
├── .gitignore
├── assets/
│   ├── icons/
│   ├── fonts/
│   └── themes/
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── state.rs
│   ├── errors.rs
│   ├── logging.rs
│   ├── config/
│   ├── widget/
│   ├── ui/
│   ├── settings/
│   ├── applications/
│   ├── platform/
│   └── services/
├── tests/
├── packaging/
└── docs/
```

## Getting Started

### Prerequisites

- Rust (latest stable toolchain)
- GTK4 / Layer Shell libraries (when building with GUI backends)

### Build and Run

```bash
cargo check
cargo build --release
cargo run
```

## Documentation

For architectural design, compatibility guidelines, and security considerations, refer to the [`docs/`](docs/) directory.
