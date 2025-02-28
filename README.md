# Flash Games Revived

A modern WebAssembly platform for Flash-like games built with Rust and Yew.

## Project Overview

Flash Games Revived aims to recreate the spirit of Flash games using modern web technologies.
By leveraging Rust and WebAssembly, we provide a secure, performant, and nostalgic gaming experience 
without the security issues of the original Flash plugin.

## Features (Planned)

- Collection of WebAssembly games inspired by classic Flash games
- Responsive design that works across devices
- Fast and secure gameplay with native performance
- No plugins required - runs in any modern browser

## Technology Stack

- **Frontend Framework**: Yew (Rust-based framework for client web apps)
- **Programming Language**: Rust
- **Compilation Target**: WebAssembly (WASM)
- **Build Tool**: Trunk (WASM application bundler)
- **Styling**: CSS (custom styles)

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Trunk](https://trunkrs.dev/) - Install with `cargo install trunk`
- [wasm32-unknown-unknown target](https://rustwasm.github.io/docs/book/game-of-life/setup.html) - Install with `rustup target add wasm32-unknown-unknown`

## Getting Started

1. Clone this repository:
   ```
   git clone https://your-repository-url/flash_games_revived.git
   cd flash_games_revived
   ```

2. Build and run the development server:
   ```
   trunk serve
   ```

3. Open your browser and navigate to `http://localhost:8080`

## Building for Production

To build the project for production:

```
trunk build --release
```

The compiled assets will be available in the `dist` directory.

## Project Structure

- `src/` - Rust source code
  - `main.rs` - Application entry point
  - `app.rs` - Main application component
  - `router.rs` - Application routing
  - `components/` - Reusable UI components
  - `pages/` - Individual page components
- `static/` - Static assets (HTML, CSS, images)
- `Trunk.toml` - Trunk configuration
- `Cargo.toml` - Rust dependencies and project configuration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.