// Font atlas data - auto-generated at compile time
include!(concat!(env!("OUT_DIR"), "/font_atlas.rs"));

use std::collections::HashMap;
use crate::renderer::GlyphMetrics;

pub struct EmbeddedAtlas {
    pub png_data: &'static [u8],
    pub glyph_coordinates: HashMap<char, GlyphMetrics>,
}

impl EmbeddedAtlas {
    pub fn new() -> Self {
        let coords = get_glyph_map();
        let mut glyph_coordinates = HashMap::new();
        
        // Convert tuple coordinates to GlyphMetrics
        for (ch, (u_min, v_min, u_max, v_max)) in coords {
            glyph_coordinates.insert(ch, GlyphMetrics {
                u_min,
                v_min,
                u_max,
                v_max,
                width: 32,
                height: 32,
            });
        }
        
        Self {
            png_data: FONT_ATLAS_PNG,
            glyph_coordinates,
        }
    }
}
