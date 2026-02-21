# AGENTS.md - Rust GUI Application Guidelines

Guidelines and standards for agentic coding in this Rust GUI application.

## Build, Lint & Test Commands

### Project Setup
```bash
# Initialize/create Cargo project (if needed)
cargo init --name matrix

# Build the project
cargo build              # Debug build
cargo build --release   # Optimized release build

# Run the application
cargo run              # Debug mode
cargo run --release    # Release mode
```

### Testing
```bash
# Run all tests
cargo test

# Run a single test
cargo test test_name -- --nocapture

# Run tests in a specific module
cargo test module_name::

# Run with output even on success
cargo test -- --nocapture

# Run tests with multiple threads (default is parallel)
cargo test -- --test-threads=1
```

### Linting & Formatting
```bash
# Format code with rustfmt
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Run clippy linter
cargo clippy

# Run clippy with all warnings treated as errors (for CI)
cargo clippy -- -D warnings

# Run all checks (format + clippy + tests)
cargo fmt && cargo clippy && cargo test
```

### Documentation
```bash
# Generate and open documentation
cargo doc --open

# Generate docs with private items
cargo doc --open --document-private-items
```

## Code Style Guidelines

### Imports
- Use alphabetical order for imports within groups
- Group imports: std library → external crates → internal modules
- Example:
  ```rust
  use std::collections::HashMap;
  use std::fs;
  
  use serde::{Deserialize, Serialize};
  use tokio::task;
  
  use crate::models::Window;
  use crate::renderer;
  ```

### Formatting & Structure
- Line length: 100 characters (soft limit, break at logical points)
- Use 4 spaces for indentation (enforced by rustfmt)
- Opening braces on same line: `fn foo() {`
- One blank line between items (fn, struct, impl blocks)

### Types & Naming
- **Functions**: `snake_case` (e.g., `render_frame()`, `handle_event()`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g., `MAX_BUFFER_SIZE`)
- **Types/Structs/Traits**: `PascalCase` (e.g., `RenderWindow`, `EventHandler`)
- **Variables**: `snake_case` (e.g., `window_size`, `is_active`)
- **Generic types**: Single uppercase letters or descriptive PascalCase (e.g., `T`, `EventType`)

### Error Handling
- Prefer `Result<T, E>` over panics in library code
- Use custom error types or `anyhow::Error` for simple cases
- Propagate errors with `?` operator
- Handle errors at appropriate boundaries
- Use `expect()` only with clear justification in comments
- Example:
  ```rust
  fn load_config(path: &str) -> Result<Config> {
      let content = std::fs::read_to_string(path)?;
      let config = serde_json::from_str(&content)?;
      Ok(config)
  }
  ```

### GUI-Specific Patterns
- Separate concerns: rendering, state management, event handling
- Use composition over inheritance for components
- Keep event loops responsive; offload heavy work to background tasks
- Use appropriate synchronization primitives (Mutex, Arc, RwLock)
- Clean up resources explicitly (especially GPU resources for GUI frameworks)

### Documentation
- Add doc comments to public items: `/// Short description`
- Use markdown in doc comments for clarity
- Document panics, errors, and safety requirements
- Example:
  ```rust
  /// Renders a single frame to the window.
  ///
  /// # Arguments
  /// * `context` - The rendering context
  ///
  /// # Returns
  /// Returns `Ok(())` on success or an error if rendering failed.
  pub fn render_frame(context: &RenderContext) -> Result<()> {
      // implementation
  }
  ```

## Project Structure (Recommended)

```
matrix/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── gui/              # GUI framework integration
│   ├── renderer/         # Rendering logic
│   ├── state/            # Application state
│   ├── events/           # Event handling
│   └── utils/            # Utilities
├── font/                 # Font assets
├── tests/                # Integration tests
├── AGENTS.md             # This file
└── README.md
```

## Dependencies (Common for Rust GUI)
- **GUI Frameworks**: `winit`, `fltk-rs`, `gtk-rs`, `iced`, or `egui`
- **Rendering**: `wgpu`, `glium`, or framework-specific
- **Serialization**: `serde`, `serde_json`
- **Async**: `tokio`, `async-std`
- **Errors**: `anyhow`, `thiserror`

## Checklist for Code Changes
- [ ] Code follows naming conventions (snake_case functions, PascalCase types)
- [ ] Imports organized and alphabetical within groups
- [ ] Error handling uses `Result` with `?` operator
- [ ] Public APIs have doc comments
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Tests pass (`cargo test`)
- [ ] For GUI changes: verified on target platform

## Tips for Agents
1. **Before making changes**: Run `cargo fmt && cargo clippy && cargo test`
2. **When adding features**: Create tests alongside implementation
3. **For GUI frameworks**: Check framework-specific patterns and examples
4. **Resource cleanup**: GUI resources (windows, textures, etc.) should be cleaned up properly
5. **Async patterns**: Use `tokio::spawn` for background tasks to keep UI responsive
6. **Platform-specific code**: Test on target platforms; document with `#[cfg(...)]` attributes
