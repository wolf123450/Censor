use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_polygon_mut;
use imageproc::point::Point;
use crate::models::{CensoringRegion, CensoringMethod};
use std::f64::consts::PI;
use base64::{engine::general_purpose, Engine as _};

pub struct ImageProcessor;

impl ImageProcessor {
    /// Load image from file path or base64
    pub fn load_image(image_input: &str) -> Result<RgbaImage, String> {
        if image_input.starts_with("data:image") {
            // Parse base64
            let parts: Vec<&str> = image_input.split(',').collect();
            if parts.len() != 2 {
                return Err("Invalid data URI format".to_string());
            }
            let base64_data = parts[1];
            let image_bytes = general_purpose::STANDARD
                .decode(base64_data)
                .map_err(|e| format!("Base64 decode failed: {}", e))?;
            let img = image::load_from_memory(&image_bytes)
                .map_err(|e| format!("Image load failed: {}", e))?
                .to_rgba8();
            Ok(img)
        } else {
            // Load from file path
            let img = image::open(image_input)
                .map_err(|e| format!("Failed to load image: {}", e))?
                .to_rgba8();
            Ok(img)
        }
    }

    /// Apply censoring to image with specified regions and methods
    pub fn apply_censoring(
        mut image: RgbaImage,
        regions: &[CensoringRegion],
    ) -> Result<RgbaImage, String> {
        for region in regions {
            match region.censoring_method {
                CensoringMethod::Blackout => {
                    Self::apply_blackout(&mut image, &region.polygon)?;
                }
                CensoringMethod::Blur => {
                    Self::apply_blur(&mut image, &region.polygon)?;
                }
                CensoringMethod::Pixelate => {
                    Self::apply_pixelate(&mut image, &region.polygon)?;
                }
            }
        }
        Ok(image)
    }

    fn apply_blackout(image: &mut RgbaImage, polygon: &[(f64, f64)]) -> Result<(), String> {
        if polygon.is_empty() {
            return Ok(());
        }

        // Convert polygon points to imageproc Point format
        let points: Vec<Point<i32>> = polygon
            .iter()
            .map(|(x, y)| Point::new(*x as i32, *y as i32))
            .collect();

        // Fill polygon with black
        draw_polygon_mut(image, &points[..], Rgba([0, 0, 0, 255]));
        Ok(())
    }

    fn apply_blur(image: &mut RgbaImage, polygon: &[(f64, f64)]) -> Result<(), String> {
        // Create a mask for the polygon area
        let (width, height) = image.dimensions();
        let mut mask = vec![vec![false; width as usize]; height as usize];

        // Fill mask using simple scanline algorithm
        Self::fill_polygon_mask(&mut mask, polygon, width, height);

        // Apply Gaussian blur to masked area
        let blur_radius = 5;
        let mut temp_image = image.clone();

        for y in 0..height {
            for x in 0..width {
                if mask[y as usize][x as usize] {
                    let mut r = 0.0;
                    let mut g = 0.0;
                    let mut b = 0.0;
                    let mut count = 0;

                    for dy in -blur_radius..=blur_radius {
                        for dx in -blur_radius..=blur_radius {
                            let nx = (x as i32 + dx).clamp(0, width as i32 - 1) as u32;
                            let ny = (y as i32 + dy).clamp(0, height as i32 - 1) as u32;

                            let pixel = image.get_pixel(nx, ny);
                            r += pixel[0] as f64;
                            g += pixel[1] as f64;
                            b += pixel[2] as f64;
                            count += 1;
                        }
                    }

                    let blurred_pixel = Rgba([
                        (r / count as f64) as u8,
                        (g / count as f64) as u8,
                        (b / count as f64) as u8,
                        image.get_pixel(x, y)[3],
                    ]);
                    temp_image.put_pixel(x, y, blurred_pixel);
                }
            }
        }

        *image = temp_image;
        Ok(())
    }

    fn apply_pixelate(image: &mut RgbaImage, polygon: &[(f64, f64)]) -> Result<(), String> {
        let (width, height) = image.dimensions();
        let mut mask = vec![vec![false; width as usize]; height as usize];

        // Fill mask using simple scanline algorithm
        Self::fill_polygon_mask(&mut mask, polygon, width, height);

        let pixel_size = 8;
        let mut temp_image = image.clone();

        for block_y in (0..height).step_by(pixel_size) {
            for block_x in (0..width).step_by(pixel_size) {
                let block_height = ((block_y + pixel_size as u32).min(height) - block_y) as usize;
                let block_width = ((block_x + pixel_size as u32).min(width) - block_x) as usize;

                // Check if any pixel in block is masked
                let mut is_masked = false;
                for y in block_y..(block_y + block_height as u32) {
                    for x in block_x..(block_x + block_width as u32) {
                        if mask[y as usize][x as usize] {
                            is_masked = true;
                            break;
                        }
                    }
                    if is_masked {
                        break;
                    }
                }

                if is_masked {
                    // Calculate average color of block
                    let mut r = 0.0;
                    let mut g = 0.0;
                    let mut b = 0.0;
                    let mut count = 0;

                    for y in block_y..(block_y + block_height as u32).min(height) {
                        for x in block_x..(block_x + block_width as u32).min(width) {
                            let pixel = image.get_pixel(x, y);
                            r += pixel[0] as f64;
                            g += pixel[1] as f64;
                            b += pixel[2] as f64;
                            count += 1;
                        }
                    }

                    let avg_pixel = Rgba([
                        (r / count as f64) as u8,
                        (g / count as f64) as u8,
                        (b / count as f64) as u8,
                        255,
                    ]);

                    // Apply average color to all pixels in block
                    for y in block_y..(block_y + block_height as u32).min(height) {
                        for x in block_x..(block_x + block_width as u32).min(width) {
                            if mask[y as usize][x as usize] {
                                temp_image.put_pixel(x, y, avg_pixel);
                            }
                        }
                    }
                }
            }
        }

        *image = temp_image;
        Ok(())
    }

    fn fill_polygon_mask(
        mask: &mut [Vec<bool>],
        polygon: &[(f64, f64)],
        width: u32,
        height: u32,
    ) {
        if polygon.len() < 3 {
            return;
        }

        let height_usize = height as usize;
        let width_usize = width as usize;

        for y in 0..height_usize {
            let mut x_intersections = vec![];

            for i in 0..polygon.len() {
                let p1 = polygon[i];
                let p2 = polygon[(i + 1) % polygon.len()];

                if ((p1.1 <= y as f64) && ((y as f64) < p2.1)) || ((p2.1 <= y as f64) && ((y as f64) < p1.1)) {
                    let x_intersection = p1.0
                        + (y as f64 - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1);
                    if x_intersection >= 0.0 && x_intersection < width as f64 {
                        x_intersections.push(x_intersection as i32);
                    }
                }
            }

            x_intersections.sort_unstable();

            for chunk in x_intersections.chunks(2) {
                if chunk.len() == 2 {
                    for x in chunk[0]..=chunk[1].min(width_usize as i32 - 1) {
                        if x >= 0 && (x as usize) < width_usize {
                            mask[y][x as usize] = true;
                        }
                    }
                }
            }
        }
    }

    /// Convert bounding box to approximate circular polygon
    pub fn bbox_to_circular_mask(x: f64, y: f64, width: f64, height: f64) -> Vec<(f64, f64)> {
        let cx = x + width / 2.0;
        let cy = y + height / 2.0;
        let rx = width / 2.0;
        let ry = height / 2.0;

        let num_points = 32;
        let mut polygon = vec![];

        for i in 0..num_points {
            let angle = 2.0 * PI * i as f64 / num_points as f64;
            let px = cx + rx * angle.cos();
            let py = cy + ry * angle.sin();
            polygon.push((px, py));
        }

        polygon
    }

    /// Convert bounding box to blob-like irregular polygon
    pub fn bbox_to_blob_mask(x: f64, y: f64, width: f64, height: f64) -> Vec<(f64, f64)> {
        let cx = x + width / 2.0;
        let cy = y + height / 2.0;

        let mut polygon = vec![];
        let num_points = 12;

        for i in 0..num_points {
            let angle = 2.0 * PI * i as f64 / num_points as f64;

            // Create irregular spacing using sine wave
            let radius_variation = 0.7 + 0.3 * (3.0 * angle).sin();
            let radius_x = width / 2.0 * radius_variation;
            let radius_y = height / 2.0 * radius_variation;

            let px = cx + radius_x * angle.cos();
            let py = cy + radius_y * angle.sin();

            polygon.push((px, py));
        }

        polygon
    }

    /// Save image to file
    pub fn save_image(image: &RgbaImage, output_path: &str) -> Result<(), String> {
        image
            .save(output_path)
            .map_err(|e| format!("Failed to save image: {}", e))
    }

    /// Convert image to base64
    pub fn image_to_base64(image: &RgbaImage) -> Result<String, String> {
        use image::ImageFormat;
        use std::io::Cursor;

        let mut buf = vec![];
        image
            .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .map_err(|e| format!("Failed to encode image: {}", e))?;

        Ok(format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(&buf)
        ))
    }
}
