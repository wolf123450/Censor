// PRODUCTION BUILD CONFIGURATION
// This file ensures optimal performance for the production release

pub mod build_config {
    pub const BLUR_RADIUS: usize = 5;
    pub const PIXELATE_SIZE: usize = 8;
    pub const POLYGON_SIMPLIFICATION_EPSILON: f64 = 1.0;
    pub const POLYGON_CIRCLE_POINTS: i32 = 32;
    pub const POLYGON_BLOB_POINTS: i32 = 12;
    pub const IMAGE_CACHE_SIZE: usize = 100; // MB
    pub const MAX_IMAGE_DIMENSIONS: u32 = 8192;
    pub const CONCURRENT_REGION_PROCESSING: usize = 4;
}
