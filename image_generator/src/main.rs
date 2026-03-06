use anyhow::{Context, Result, anyhow};
use image::{DynamicImage, ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale, point};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    slug: String,
}

const HOST_TEXT: &str = "learning-rust.github.io";
const DOCS_DIR: &str = "../content/en/docs";
const OUTPUT_DOCS_DIR: &str = "../docs/docs";

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 630;
const PADDING: u32 = 32;

const OUTPUT_JPEG_QUALITY: f32 = 85.0;

const FONT_PATH: &str = "fonts/Lato-Regular.ttf";
const LOGO_IMAGE_PATH: &str = "logo.png";
const SECONDARY_IMAGE_PATH: &str = "secondary.png";

const TEXT_COLOR: Rgba<u8> = Rgba([255, 255, 255, 255]); // White
const GRADIENT_COLORS: [Rgba<u8>; 5] = [
    Rgba([0x86, 0x6e, 0xe7, 255]), // #866ee7
    Rgba([0xea, 0x60, 0xda, 255]), // #ea60da
    Rgba([0xed, 0x8f, 0x57, 255]), // #ed8f57
    Rgba([0xfb, 0xd4, 0x1d, 255]), // #fbd41d
    Rgba([0x2c, 0xca, 0x91, 255]), // #2cca91
];

const LEFT_SECTION_WIDTH_RATIO: f32 = 0.40;
const LOGO_TARGET_HEIGHT: u32 = 48;

const HOST_TEXT_SCALE: f32 = 28.0;
const PAGE_TITLE_SCALE: f32 = 85.0;

fn main() -> Result<()> {
    let font = load_font(FONT_PATH)?;
    let mut processed_count = 0;
    for entry in markdown_files(DOCS_DIR) {
        match process_markdown_file(entry.path(), &font) {
            Ok(()) => {
                processed_count += 1;
                println!("✓ Generated OG image for: {}", entry.path().display());
            }
            Err(e) => {
                eprintln!("✗ Failed to process {}: {}", entry.path().display(), e);
            }
        }
    }

    println!("\nProcessed {} markdown files", processed_count);
    Ok(())
}

fn load_font(font_path: &str) -> Result<Font<'static>> {
    let font_data =
        fs::read(font_path).with_context(|| format!("Could not read font file: {}", font_path))?;

    Font::try_from_vec(font_data)
        .ok_or_else(|| anyhow!("Could not parse font data from {}", font_path))
}

fn markdown_files(root: &str) -> impl Iterator<Item = walkdir::DirEntry> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
}

fn process_markdown_file(file_path: &Path, font: &Font) -> Result<()> {
    // Read the markdown file
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    // Parse frontmatter
    let frontmatter = parse_frontmatter(&content)
        .with_context(|| format!("Failed to parse frontmatter from: {}", file_path.display()))?;

    // Generate the image
    let img = generate_image(&frontmatter.title, font)?;
    let output_path = output_image_path(&frontmatter.slug);

    let rgb = DynamicImage::ImageRgba8(img).to_rgb8();

    let buf = std::panic::catch_unwind(|| -> std::io::Result<Vec<u8>> {
        let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);

        comp.set_size(rgb.width() as usize, rgb.height() as usize);
        comp.set_quality(OUTPUT_JPEG_QUALITY);
        comp.set_progressive_mode();

        let mut comp = comp.start_compress(Vec::new())?;

        // mozjpeg expects rows (scanlines)
        let row_stride = (rgb.width() as usize) * 3;
        for row in rgb.as_raw().chunks_exact(row_stride) {
            comp.write_scanlines(row)?;
        }

        comp.finish()
    })
    .map_err(|_| anyhow::anyhow!("mozjpeg panicked while encoding {}", output_path.display()))??;

    std::fs::write(&output_path, &buf)
        .with_context(|| format!("Failed to save image: {}", output_path.display()))?;

    Ok(())
}

fn output_image_path(slug: &str) -> PathBuf {
    Path::new(OUTPUT_DOCS_DIR)
        .join(Path::new(slug))
        .join("og.jpg")
}

fn parse_frontmatter(content: &str) -> Result<Frontmatter> {
    // Find the frontmatter section (between --- lines)
    if !content.starts_with("---") {
        anyhow::bail!("File does not start with frontmatter delimiter");
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut end_index = None;

    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            end_index = Some(i);
            break;
        }
    }

    let end_index =
        end_index.ok_or_else(|| anyhow::anyhow!("Could not find end of frontmatter"))?;

    // Extract frontmatter content (skip the first --- line)
    let frontmatter_content = lines[1..end_index].join("\n");

    // Parse YAML
    serde_yaml::from_str(&frontmatter_content).context("Failed to parse frontmatter as YAML")
}

fn generate_image(page_title: &str, font: &Font) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut img = generate_gradient_background(WIDTH, HEIGHT);

    // --- Layout Calculations ---
    let left_section_width = (WIDTH as f32 * LEFT_SECTION_WIDTH_RATIO) as u32;
    let right_section_width = WIDTH - left_section_width;

    let mut current_y_offset = PADDING as i32;

    // --- Left Section: Top Row (Logo & Host Text) ---
    let mut logo_final_width = 0;
    let logo_render_x_offset = PADDING as i64;

    if !LOGO_IMAGE_PATH.is_empty() {
        if let Ok(logo_img_dyn) = image::open(LOGO_IMAGE_PATH) {
            let logo_img_orig = logo_img_dyn.to_rgba8();
            let (orig_w, orig_h) = logo_img_orig.dimensions();

            let logo_final_height = LOGO_TARGET_HEIGHT;
            logo_final_width = (orig_w as f32 * (logo_final_height as f32 / orig_h as f32)) as u32;

            let resized_logo = image::imageops::resize(
                &logo_img_orig,
                logo_final_width,
                logo_final_height,
                image::imageops::FilterType::Lanczos3,
            );

            image::imageops::overlay(
                &mut img,
                &resized_logo,
                logo_render_x_offset,
                current_y_offset as i64,
            );
        } else {
            eprintln!(
                "Warning: Logo image not found at {}. Skipping.",
                LOGO_IMAGE_PATH
            );
        }
    }

    // Host Text
    let host_text_scale = Scale::uniform(HOST_TEXT_SCALE);
    let (_, host_text_height) = measure_text_size(HOST_TEXT, host_text_scale, font);

    let host_text_render_x =
        logo_render_x_offset + logo_final_width as i64 + (PADDING / 2) as i64 / 2;
    let host_text_render_y =
        current_y_offset + (LOGO_TARGET_HEIGHT as i32 / 2) - (host_text_height as i32 / 2);

    draw_text_mut(
        &mut img,
        TEXT_COLOR,
        host_text_render_x as i32,
        host_text_render_y as i32,
        host_text_scale,
        font,
        HOST_TEXT,
    );

    // Update y_offset after logo and host text row
    let top_row_height = LOGO_TARGET_HEIGHT.max(host_text_height) as i32;
    current_y_offset += top_row_height + PADDING as i32;

    // --- Left Section: Page Title ---
    let page_title_scale = Scale::uniform(PAGE_TITLE_SCALE);
    let page_title_render_x = PADDING as i32;

    let max_title_width_for_wrap = left_section_width - (PADDING * 2);

    let words = page_title.split_whitespace();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        let (test_width, _) = measure_text_size(&test_line, page_title_scale, font);

        if test_width <= max_title_width_for_wrap {
            current_line = test_line;
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    let line_height_val = font.v_metrics(page_title_scale).line_gap
        + font.v_metrics(page_title_scale).ascent
        - font.v_metrics(page_title_scale).descent;

    for (i, line) in lines.iter().enumerate() {
        draw_text_mut(
            &mut img,
            TEXT_COLOR,
            page_title_render_x,
            current_y_offset + (i as f32 * line_height_val) as i32,
            page_title_scale,
            font,
            line,
        );
    }

    // --- Right Section: Secondary Image (Oversized with hidden overflow, top-left shown) ---
    let secondary_section_x_start = left_section_width as i64;
    let secondary_section_y_start = (PADDING * 3) as i64; // Apply top padding to the section itself
    let secondary_section_width = right_section_width;
    let secondary_section_height = HEIGHT - PADDING; // No bottom padding, extends to full height minus top padding

    if let Ok(secondary_img_orig_dyn) = image::open(SECONDARY_IMAGE_PATH) {
        let secondary_img_orig = secondary_img_orig_dyn.to_rgba8();
        let (orig_w, orig_h) = secondary_img_orig.dimensions();
        let resized_secondary = image::imageops::resize(
            &secondary_img_orig,
            orig_w,
            orig_h,
            image::imageops::FilterType::Lanczos3,
        );

        // Create a temporary buffer for the right section
        let mut right_section_buffer =
            ImageBuffer::new(secondary_section_width, secondary_section_height);

        // Place the top-left of the oversized image at the top-left of the buffer
        // This will effectively show the top-left part of the secondary image,
        // with overflow hidden.
        let render_x_in_buffer = 0;
        let render_y_in_buffer = 0;

        image::imageops::overlay(
            &mut right_section_buffer,
            &resized_secondary,
            render_x_in_buffer,
            render_y_in_buffer,
        );

        // Overlay the clipped right_section_buffer onto the main image
        image::imageops::overlay(
            &mut img,
            &right_section_buffer,
            secondary_section_x_start,
            secondary_section_y_start,
        );
    } else {
        eprintln!(
            "Warning: Secondary image not found at {}. Skipping right section.",
            SECONDARY_IMAGE_PATH
        );
    }

    Ok(img)
}

fn generate_gradient_background(width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);
    let num_stops = (GRADIENT_COLORS.len() - 1) as f32;

    for y in 0..height {
        for x in 0..width {
            let diagonal_pos = (x as f32 / width as f32 + y as f32 / height as f32) / 2.0;

            let stop_index = (diagonal_pos * num_stops).floor() as usize;
            let local_pos = (diagonal_pos * num_stops) % 1.0;

            let color1 = GRADIENT_COLORS[stop_index];
            let color2 = GRADIENT_COLORS[(stop_index + 1).min(GRADIENT_COLORS.len() - 1)];

            let r = (color1[0] as f32 * (1.0 - local_pos) + color2[0] as f32 * local_pos) as u8;
            let g = (color1[1] as f32 * (1.0 - local_pos) + color2[1] as f32 * local_pos) as u8;
            let b = (color1[2] as f32 * (1.0 - local_pos) + color2[2] as f32 * local_pos) as u8;
            let a = (color1[3] as f32 * (1.0 - local_pos) + color2[3] as f32 * local_pos) as u8;

            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }
    img
}

fn measure_text_size(text: &str, scale: Scale, font: &Font) -> (u32, u32) {
    let v_metrics = font.v_metrics(scale);
    let height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap).ceil() as u32;
    let mut width: f32 = 0.0;

    for glyph in font.layout(text, scale, point(0.0, 0.0)) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            width = width.max(bounding_box.max.x as f32);
        }
    }
    (width as u32, height)
}
