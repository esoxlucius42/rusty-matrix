use ab_glyph::{Font, FontRef, PxScale};
use image::{ImageBuffer, ImageEncoder, Rgba, RgbaImage};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let font_path = "font/PleckJP-Regular.ttf";
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let atlas_path = Path::new(&out_dir).join("font_atlas.rs");

    // Half-width katakana: U+FF66 to U+FF9D (58 characters)
    let charset: String = (0xFF66..=0xFF9D)
        .filter_map(char::from_u32)
        .collect();

    // Load font
    let font_data = std::fs::read(font_path).expect("Failed to read font file");
    let font = FontRef::try_from_slice(&font_data).expect("Failed to parse font");

    // Atlas configuration
    const ATLAS_WIDTH: u32 = 2048;
    const ATLAS_HEIGHT: u32 = 2048;
    const GLYPH_SIZE: u32 = 32;
    const PADDING: u32 = 4;

    // Create atlas
    let mut atlas: RgbaImage = ImageBuffer::new(ATLAS_WIDTH, ATLAS_HEIGHT);
    
    // Fill with black background
    for pixel in atlas.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]); // Transparent black
    }

    let mut glyph_map: HashMap<char, (f32, f32, f32, f32)> = HashMap::new();
    let scale = PxScale::from(GLYPH_SIZE as f32);

    let mut current_x = PADDING;
    let mut current_y = PADDING;
    let mut glyph_count = 0;
    let mut failed_count = 0;
    let mut failed_chars = Vec::new();

    for ch in charset.chars() {
        // Check if we need to move to next row
        if current_x + GLYPH_SIZE + PADDING > ATLAS_WIDTH {
            current_x = PADDING;
            current_y += GLYPH_SIZE + PADDING;

            if current_y + GLYPH_SIZE + PADDING > ATLAS_HEIGHT {
                eprintln!("Warning: Font atlas full, skipping remaining characters");
                break;
            }
        }

        // Rasterize glyph
        let glyph_id = font.glyph_id(ch);
        let glyph = glyph_id.with_scale_and_position(scale, ab_glyph::Point { x: 0.0, y: 0.0 });

        if let Some(outlined) = font.outline_glyph(glyph) {
            // Create glyph bitmap
            let mut glyph_img: RgbaImage = ImageBuffer::new(GLYPH_SIZE, GLYPH_SIZE);
            for pixel in glyph_img.pixels_mut() {
                *pixel = Rgba([0, 0, 0, 0]);
            }

            // Rasterize outline
            outlined.draw(|x: u32, y: u32, coverage: f32| {
                if x < GLYPH_SIZE && y < GLYPH_SIZE {
                    let alpha = (coverage * 255.0) as u8;
                    *glyph_img.get_pixel_mut(x, y) = Rgba([255, 255, 255, alpha]);
                }
            });

            // Copy to atlas
            for y in 0..GLYPH_SIZE {
                for x in 0..GLYPH_SIZE {
                    let src = *glyph_img.get_pixel(x, y);
                    let dst_x = current_x + x;
                    let dst_y = current_y + y;
                    
                    if dst_x < ATLAS_WIDTH && dst_y < ATLAS_HEIGHT {
                        *atlas.get_pixel_mut(dst_x, dst_y) = src;
                    }
                }
            }

            glyph_count += 1;

            // Store glyph metrics (normalized UV coordinates) - ONLY for successfully rasterized glyphs
            let u_min = current_x as f32 / ATLAS_WIDTH as f32;
            let v_min = current_y as f32 / ATLAS_HEIGHT as f32;
            let u_max = (current_x + GLYPH_SIZE) as f32 / ATLAS_WIDTH as f32;
            let v_max = (current_y + GLYPH_SIZE) as f32 / ATLAS_HEIGHT as f32;

            glyph_map.insert(ch, (u_min, v_min, u_max, v_max));
        } else {
            // Character failed to rasterize
            failed_count += 1;
            failed_chars.push(ch);
        }

        current_x += GLYPH_SIZE + PADDING;
    }

    // Encode atlas as PNG to bytes
    let png_bytes = {
        let mut buffer = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        encoder
            .write_image(
                atlas.as_raw(),
                ATLAS_WIDTH,
                ATLAS_HEIGHT,
                image::ColorType::Rgba8,
            )
            .expect("Failed to encode PNG");
        buffer
    };

    println!("Generated font atlas with {} glyphs in PNG ({} bytes)", glyph_count, png_bytes.len());

    // Generate Rust code with embedded atlas and glyph map
    let mut output = String::new();
    output.push_str("// Auto-generated font atlas - do not edit\n\n");
    output.push_str("pub const FONT_ATLAS_PNG: &[u8] = &[\n");

    // Write PNG bytes as hex
    for (i, byte) in png_bytes.iter().enumerate() {
        if i % 16 == 0 {
            output.push_str("    ");
        }
        output.push_str(&format!("0x{:02x}, ", byte));
        if (i + 1) % 16 == 0 {
            output.push('\n');
        }
    }
    if png_bytes.len() % 16 != 0 {
        output.push('\n');
    }
    output.push_str("];\n\n");

    // Write glyph map
    output.push_str("pub fn get_glyph_map() -> std::collections::HashMap<char, (f32, f32, f32, f32)> {\n");
    output.push_str("    let mut map = std::collections::HashMap::new();\n");

    for (ch, (u_min, v_min, u_max, v_max)) in &glyph_map {
        let ch_escaped = format!("{:?}", ch);
        output.push_str(&format!(
            "    map.insert({}, ({}, {}, {}, {}));\n",
            ch_escaped, u_min, v_min, u_max, v_max
        ));
    }

    output.push_str("    map\n");
    output.push_str("}\n");

    // Write to file
    let mut file = File::create(&atlas_path).expect("Failed to create output file");
    file.write_all(output.as_bytes())
        .expect("Failed to write output file");

    // Print summary
    println!("cargo:warning=Font atlas generated: {} glyphs successfully rasterized", glyph_count);
    if failed_count > 0 {
        println!("cargo:warning=WARNING: {} glyphs failed to rasterize:", failed_count);
        for ch in &failed_chars {
            println!("cargo:warning=  - '{}' (U+{:04X})", ch, *ch as u32);
        }
    }
}
