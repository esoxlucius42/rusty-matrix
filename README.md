# Matrix Digital Rain - wgpu Edition

A high-performance Matrix digital rain application written in Rust using **wgpu** for cross-platform GPU rendering.

## Features

✨ **Digital Rain Animation**
- Authentic Matrix-style characters (Japanese katakana + ASCII)
- Falling character columns with smooth animation
- Configurable rain speed and density

🎨 **Color System**
- Leading character rendered in white (bright)
- Trailing characters fade from bright green → dark green
- Creates authentic "dripping" visual effect

🖥️ **Window Management**
- Resizable window (1280x720 default)
- **F11** - Toggle fullscreen mode
- **ESC** - Exit fullscreen (or close app in windowed mode)
- Black background for authentic Matrix feel

⚡ **Performance**
- Cross-platform GPU rendering via wgpu (Vulkan on Linux, Metal on macOS, DX12 on Windows)
- GPU-accelerated graphics pipeline
- MAILBOX present mode for reduced frame latency
- Efficient vertex/index buffer management

## Build Requirements

- **Rust** 1.70+ (2021 edition)
- **Linux/Windows/Mac** with GPU support (Vulkan, Metal, or DX12)
- No separate GPU driver SDK required (handled by wgpu)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## Project Structure

```
matrix/
├── src/
│   ├── main.rs          # Entry point, window creation
│   ├── gui.rs           # Event handling, window management
│   ├── renderer.rs      # wgpu GPU rendering pipeline
│   ├── rain.rs          # Digital rain simulation logic
│   └── events.rs        # Event handling utilities
├── shaders/
│   └── shader.wgsl      # WGSL vertex and fragment shaders
├── font/
│   └── matrix code nfi.ttf  # Matrix font (for future text rendering)
├── Cargo.toml           # Dependencies
├── .gitignore           # Git ignore rules
└── AGENTS.md            # Guidelines for AI agents
```

## Dependencies

- **wgpu** - Cross-platform GPU abstraction (Vulkan/Metal/DX12)
- **winit** - Cross-platform window creation and event handling
- **bytemuck** - Type casting for GPU data
- **rand** - Random character generation
- **naga** - Shader translation (WGSL support)
- **pollster** - Async runtime for GPU initialization

## Architecture

```
Window (winit)
    ↓
GPU Instance (wgpu) → Device Selection
    ↓
Surface Creation (cross-platform)
    ↓
Queue & Command Encoding
    ↓
Render Pass → Graphics Pipeline
    ↓
Vertex/Index Buffers → GPU Upload
    ↓
Command Buffer Submission & Present
```

## Rendering Pipeline

1. **Shader Compilation**: WGSL shaders compiled at runtime by naga
2. **Vertex Input**: Position (2D), UV (texture coords), Color (RGBA)
3. **Rasterization**: CCW winding, back-face culling
4. **Blending**: Alpha blending for character transparency
5. **Presentation**: Mailbox present mode (low latency, vsync-optional)

## Performance Optimizations

- Cross-platform GPU abstraction with automatic backend selection
- MAILBOX present mode reduces frame latency
- Efficient quad-based character rendering
- Optimal memory layout for GPU cache performance
- Release build optimizations: opt-level 3, LTO, single codegen unit

## Controls

| Key | Action |
|-----|--------|
| **F11** | Toggle fullscreen |
| **ESC** | Exit fullscreen / Quit app |
| **Close Button** | Quit app |
| **Resize** | Window resizes with GPU reinitialization |

## Technical Details

### Vertex Format
```rust
struct Vertex {
    position: [f32; 2],  // NDC coordinates (-1 to 1)
    uv: [f32; 2],        // Texture coordinates (0 to 1)
    color: [f32; 4],     // RGBA (white leading, green trailing)
}
```

### Rain Simulation
- Raindrops spawn at random X positions
- Each drop has configurable length (10-30 chars) and speed (1-3)
- Characters update every frame
- Drops removed when off-screen, new ones spawn

### Color Gradient
- **Leading character**: Pure white `[1.0, 1.0, 1.0, 1.0]`
- **Trailing chars**: Green with brightness fade `[0.0, brightness, 0.0, brightness]`
- Brightness = `(distance_from_head / length) * 0.7 + 0.1`

## Future Enhancements

- [ ] Proper font rasterization using the Matrix font file
- [ ] Multiple render passes for post-processing effects
- [ ] Glow/bloom effects for authentic Matrix look
- [ ] Configurable rain speed and density via UI
- [ ] Screenshot functionality
- [ ] Recording to video file

## License

Open source - use freely for learning and experimentation.
