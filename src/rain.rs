use rand::Rng;

// Re-export for use in renderer
pub use crate::renderer::{GlyphMetrics, Vertex};

#[derive(Clone, Copy, Debug)]
pub struct Raindrop {
    pub x: usize,
    pub y: i32,
    pub length: usize,
    pub speed: f32,
    pub chars: [char; 32],
    pub char_count: usize,
}

pub struct RainSimulation {
    raindrops: Vec<Raindrop>,
    width: usize,
    height: usize,
    virtual_height: usize,
    frame_count: u32,
    rng: rand::rngs::ThreadRng,
    charset: Vec<char>,
}

// Half-width katakana: U+FF66 to U+FF9D (58 characters)
fn get_charset() -> Vec<char> {
    (0xFF66..=0xFF9D)
        .filter_map(char::from_u32)
        .collect()
}

// Regenerate character chain for recycled raindrops
fn regenerate_chars(raindrop: &mut Raindrop, charset: &[char], rng: &mut rand::rngs::ThreadRng) {
    raindrop.chars = [' '; 32];
    raindrop.char_count = 0;
    let new_length = rng.gen_range(10..30);
    raindrop.length = new_length;
    
    for _ in 0..new_length.min(32) {
        let char_idx = rng.gen_range(0..charset.len());
        raindrop.chars[raindrop.char_count] = charset[char_idx];
        raindrop.char_count += 1;
    }
}

impl RainSimulation {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sim = Self {
            raindrops: Vec::new(),
            width,
            height,
            virtual_height: height * 3,
            frame_count: 0,
            rng: rand::thread_rng(),
            charset: get_charset(),
        };
        sim.spawn_raindrops();
        sim
    }

    fn spawn_raindrops(&mut self) {
        // Create initial raindrops across the width, starting above screen
        for x in (0..self.width).step_by(20) {
            self.create_raindrop(x);
        }
    }

    fn create_raindrop(&mut self, x: usize) {
        let length = self.rng.gen_range(10..30);
        
        // Weighted speed distribution: biased toward faster speeds
        // Sum of two ranges (2.0-4.0 + 0.0-1.0) = 2.0-5.0 with higher average
        let base_speed = self.rng.gen_range(2.0..4.0);
        let boost = self.rng.gen_range(0.0..1.0);
        let speed = base_speed + boost;

        let mut chars = [' '; 32];
        let mut char_count = 0;
        for _ in 0..length.min(32) {
            let char_idx = self.rng.gen_range(0..self.charset.len());
            chars[char_count] = self.charset[char_idx];
            char_count += 1;
        }

        // Randomize spawn Y across entire virtual area (3x height)
        let random_spawn_offset = self.rng.gen_range(0..=(self.height as i32 * 3));
        let spawn_y = -(self.height as i32) + random_spawn_offset;

        self.raindrops.push(Raindrop {
            x,
            y: spawn_y,
            length,
            speed,
            chars,
            char_count,
        });
    }

    pub fn update(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);

        for raindrop in &mut self.raindrops {
            // Direct pixel movement per frame, weighted toward faster speeds
            raindrop.y += raindrop.speed as i32;
        }

        // Recycle raindrops that exit bottom of screen (not removal)
        for raindrop in &mut self.raindrops {
            if raindrop.y > (self.height as i32 * 2) {
                // Recycle: reset to top of virtual area and randomize
                raindrop.y = -(self.height as i32);
                raindrop.x = self.rng.gen_range(0..self.width);
                regenerate_chars(raindrop, &self.charset, &mut self.rng);
            }
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.virtual_height = height * 3;
        self.raindrops.clear();
        self.charset = get_charset();
        self.spawn_raindrops();
    }

    pub fn generate_vertex_data(
        &self,
        glyph_map: &std::collections::HashMap<char, GlyphMetrics>,
    ) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let width_f32 = self.width as f32;
        let height_f32 = self.height as f32;

        // Debug: count lookups and misses
        let mut total_chars = 0;
        let mut found_chars = 0;
        let mut missed_chars = std::collections::HashSet::new();

        for raindrop in &self.raindrops {
            for (char_idx, &ch) in raindrop.chars[..raindrop.char_count].iter().enumerate() {
                total_chars += 1;
                
                // Get glyph metrics
                let glyph_metrics = match glyph_map.get(&ch) {
                    Some(m) => {
                        found_chars += 1;
                        m
                    }
                    None => {
                        missed_chars.insert(ch);
                        continue; // Skip if glyph not available
                    }
                };

                // Calculate Y position for this character
                let char_y = raindrop.y as f32 - (char_idx as f32 * 16.0);

                // Skip if off-screen (with padding for smooth culling)
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

        // Debug output: show lookup statistics
        if total_chars > 0 {
            eprintln!(
                "[Vertex Gen] Total chars: {}, Found: {}, Missed: {} ({:.1}% hit rate)",
                total_chars,
                found_chars,
                missed_chars.len(),
                (found_chars as f32 / total_chars as f32) * 100.0
            );
            if !missed_chars.is_empty() {
                let mut missed_list: Vec<char> = missed_chars.into_iter().collect();
                missed_list.sort();
                eprintln!("[Vertex Gen] Missing chars: {:?}", missed_list);
            }
        }

        (vertices, indices)
    }
}
