// Rain data structures for compute shader
struct Raindrop {
    x: f32,
    y: f32,
    speed: f32,
    char_index: u32,
    char_count: u32,
    _padding: vec2u,
};

// Uniform structure with all parameters
struct RainUniforms {
    time: u32,
    window_height: u32,
    rain_count: u32,
    _padding: u32,
};

// Storage buffers for compute shader
@group(0) @binding(0) var<storage, read_write> raindrops: array<Raindrop>;
@group(0) @binding(1) var<uniform> uniforms: RainUniforms;

// Compute shader for rain updates
@compute @workgroup_size(256)
fn cs_update_rain(@builtin(global_invocation_id) global_id: vec3u) {
    let idx = global_id.x;
    if (idx >= uniforms.rain_count) {
        return;
    }

    var drop = raindrops[idx];
    
    // Update position based on speed
    drop.y += drop.speed;
    
    // Wrap around when off-screen
    if (drop.y > f32(uniforms.window_height) + 100.0) {
        drop.y = -100.0;
        drop.char_index = (drop.char_index + 7u) % drop.char_count;
    }
    
    // Change character occasionally
    if ((uniforms.time / 5u) % 3u == 0u) {
        drop.char_index = (drop.char_index + 1u) % drop.char_count;
    }
    
    raindrops[idx] = drop;
}

struct VertexInput {
    @location(0) position: vec2f,
    @location(1) uv: vec2f,
    @location(2) color: vec4f,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
    @location(1) uv: vec2f,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4f(input.position, 0.0, 1.0);
    output.color = input.color;
    output.uv = input.uv;
    return output;
}

@group(0) @binding(0) var glyph_texture: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {
    // Sample the glyph texture
    let glyph_color = textureSample(glyph_texture, tex_sampler, input.uv);
    
    // Multiply by vertex color for green gradient effect
    let final_color = input.color * glyph_color;
    
    return final_color;
}
