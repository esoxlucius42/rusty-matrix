use rand::Rng;

// Re-export for use in renderer
pub use crate::renderer::{GlyphMetrics, Vertex};

#[derive(Clone, Copy, Debug)]
pub struct Raindrop {
    pub x: usize,
    pub y: usize,
    pub length: usize,
    pub speed: usize,
    pub chars: [char; 32],
    pub char_count: usize,
}

pub struct RainSimulation {
    raindrops: Vec<Raindrop>,
    width: usize,
    height: usize,
    frame_count: u32,
    rng: rand::rngs::ThreadRng,
}

const CHARSET: &str = "ﾊﾐﾋｰｳﾆｻﾓﾗﾔﾏﾗﾁﾔﾜﾂｦﾘﾅﾆﾁﾎﾓﾆﾊﾐﾊﾁﾈﾌﾆﾈﾊﾐﾊﾏﾁﾔﾆﾘｦﾊﾏﾓﾈﾓﾅﾔﾏﾛﾇﾎﾜﾘﾍﾑﾀﾘﾅﾑﾊﾐﾎﾀﾏﾂｻﾗﾊﾈﾌﾊﾓﾐﾈﾁﾋﾋﾄﾁﾎﾈﾐﾜﾀﾌﾐﾔﾏﾊﾄﾂﾊﾏﾁﾔﾃﾏﾊﾊﾆﾈﾊﾐﾎﾊﾏﾐﾋﾓﾋﾎﾌﾆﾔﾀｦﾐﾜﾇﾛﾛﾌﾍﾘﾓﾆﾘﾃﾌﾊﾀﾉﾎﾅﾑﾓﾓﾏﾗﾎﾏﾁﾊﾜﾃﾌﾓﾊﾊﾑﾈﾊﾂﾃﾌﾊﾁﾔﾀﾊﾂﾘﾏﾎﾊﾊﾌﾋﾉﾋﾀﾌﾜﾀﾀﾆﾈﾌﾔﾀﾘﾂﾔﾘﾌﾀﾆﾌﾄﾂﾋﾜﾉﾐﾈﾂﾂﾋﾄﾀﾏﾁﾜﾃﾌﾄﾂﾄﾀﾘﾋﾠﾏﾁﾀﾀﾏﾀﾇﾅﾄﾃﾀﾘﾆﾘﾄﾂﾊﾂﾅﾈﾂﾕﾜﾓﾘﾆﾊﾂﾜﾊﾃﾀﾍﾌﾜﾛﾕﾊ0123456789:・\"'.,-ﾞﾟ";

impl RainSimulation {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sim = Self {
            raindrops: Vec::new(),
            width,
            height,
            frame_count: 0,
            rng: rand::thread_rng(),
        };
        sim.spawn_raindrops();
        sim
    }

    fn spawn_raindrops(&mut self) {
        // Create initial raindrops across the width
        for x in (0..self.width).step_by(20) {
            self.create_raindrop(x);
        }
    }

    fn create_raindrop(&mut self, x: usize) {
        let length = self.rng.gen_range(10..30);
        let speed = self.rng.gen_range(1..4);

        let mut chars = [' '; 32];
        let mut char_count = 0;
        for _ in 0..length.min(32) {
            let char_idx = self.rng.gen_range(0..CHARSET.len());
            chars[char_count] = CHARSET.chars().nth(char_idx).unwrap_or('a');
            char_count += 1;
        }

        self.raindrops.push(Raindrop {
            x,
            y: 0,
            length,
            speed,
            chars,
            char_count,
        });
    }

    pub fn update(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);

        for raindrop in &mut self.raindrops {
            if self.frame_count % (5 - raindrop.speed.min(4)) as u32 == 0 {
                raindrop.y += 1;
            }
        }

        // Remove raindrops that are off screen and create new ones
        let mut i = 0;
        while i < self.raindrops.len() {
            if self.raindrops[i].y > self.height + self.raindrops[i].length {
                self.raindrops.remove(i);
            } else {
                i += 1;
            }
        }

        // Spawn new raindrops occasionally
        if self.frame_count % 5 == 0 && self.raindrops.len() < (self.width / 15) {
            let x = self.rng.gen_range(0..self.width);
            self.create_raindrop(x);
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.raindrops.clear();
        self.spawn_raindrops();
    }

    pub fn raindrops(&self) -> &[Raindrop] {
        &self.raindrops
    }

    pub fn generate_vertex_data(
        &self,
        glyph_map: &std::collections::HashMap<char, GlyphMetrics>,
    ) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let width_f32 = self.width as f32;
        let height_f32 = self.height as f32;

        for raindrop in &self.raindrops {
            for (char_idx, &ch) in raindrop.chars[..raindrop.char_count].iter().enumerate() {
                // Get glyph metrics
                let glyph_metrics = match glyph_map.get(&ch) {
                    Some(m) => m,
                    None => continue, // Skip if glyph not available
                };

                // Calculate Y position for this character
                let char_y = raindrop.y as f32 - (char_idx as f32 * 16.0);

                // Skip if off-screen
                if char_y < -50.0 || char_y > height_f32 + 50.0 {
                    continue;
                }

                // Calculate color: white for head, fade to green for tail
                let distance_from_head = char_idx as f32;
                let max_distance = raindrop.length as f32;
                let brightness = (1.0 - (distance_from_head / max_distance)) * 0.7 + 0.1;
                let brightness = brightness.clamp(0.0, 1.0);

                let color = if char_idx == 0 {
                    // Head: pure white
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    // Tail: green fade
                    [
                        brightness * 0.1,
                        brightness * 1.0,
                        brightness * 0.1,
                        brightness,
                    ]
                };

                // Convert pixel coords to NDC
                let x_pixel = raindrop.x as f32;
                let x_ndc = (2.0 * x_pixel / width_f32) - 1.0;
                let y_ndc = 1.0 - (2.0 * char_y / height_f32);

                // Glyph quad width and height in NDC
                let glyph_width_ndc = (2.0 * glyph_metrics.width as f32) / width_f32;
                let glyph_height_ndc = (2.0 * glyph_metrics.height as f32) / height_f32;

                // Add quad vertices (2 triangles)
                let base_idx = vertices.len() as u32;

                // Bottom-left
                vertices.push(Vertex {
                    position: [x_ndc, y_ndc - glyph_height_ndc],
                    uv: [glyph_metrics.u_min, glyph_metrics.v_max],
                    color,
                });

                // Bottom-right
                vertices.push(Vertex {
                    position: [x_ndc + glyph_width_ndc, y_ndc - glyph_height_ndc],
                    uv: [glyph_metrics.u_max, glyph_metrics.v_max],
                    color,
                });

                // Top-left
                vertices.push(Vertex {
                    position: [x_ndc, y_ndc],
                    uv: [glyph_metrics.u_min, glyph_metrics.v_min],
                    color,
                });

                // Top-right
                vertices.push(Vertex {
                    position: [x_ndc + glyph_width_ndc, y_ndc],
                    uv: [glyph_metrics.u_max, glyph_metrics.v_min],
                    color,
                });

                // First triangle (bottom-left, bottom-right, top-left)
                indices.push(base_idx);
                indices.push(base_idx + 1);
                indices.push(base_idx + 2);

                // Second triangle (bottom-right, top-right, top-left)
                indices.push(base_idx + 1);
                indices.push(base_idx + 3);
                indices.push(base_idx + 2);
            }
        }

        (vertices, indices)
    }
}
