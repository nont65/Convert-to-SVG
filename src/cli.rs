use clap::{Parser, ValueEnum};
use colored::*;
use std::path::PathBuf;
use visioncortex::PathSimplifyMode;

#[derive(Debug, Clone, ValueEnum)]
pub enum Mode {
    /// Smooth curves (spline)
    Spline,
    /// Sharp edges (polygon)
    Polygon,
    /// No simplification
    None,
}

impl Mode {
    pub fn to_path_simplify_mode(&self) -> PathSimplifyMode {
        match self {
            Mode::Spline => PathSimplifyMode::Spline,
            Mode::Polygon => PathSimplifyMode::Polygon,
            Mode::None => PathSimplifyMode::None,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ColorMode {
    /// Full Color
    Color,
    /// Black and white (binary)
    Binary,
}

impl ColorMode {
    pub fn to_vtracer_color_mode(&self) -> vtracer::ColorMode {
        match self {
            ColorMode::Color => vtracer::ColorMode::Color,
            ColorMode::Binary => vtracer::ColorMode::Binary,
        }
    }
}

#[derive(Parser)]
#[command(name = "convert-to-svg")]
#[command(about = "Convert JPG, PNG, or WebP images to SVG", long_about = None)]
pub struct Cli {
    /// Input image file (jpg, png, webp)
    #[arg(short, long, value_name = "FILE", value_parser = validate_image_extension)]
    pub input: PathBuf,

    /// Output SVG file
    #[arg(short, long, value_name = "FILE", value_parser = validate_svg_extension)]
    pub output: Option<PathBuf>,

    /// Color mode
    #[arg(long, value_enum, default_value = "color")]
    pub color_mode: ColorMode,

    /// Color precision (0-255, higher = more colors)
    #[arg(long, default_value_t = 8, value_parser = clap::value_parser!(i32).range(0..=255))]
    pub color_precision: i32,

    /// Path precision (1-20, higher = more detail)
    #[arg(long, default_value_t = 10, value_parser = clap::value_parser!(u32).range(1..=20))]
    pub path_precision: u32,

    /// Filter speckle (0-10, lower = more detail)
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(0..=10))]
    pub filter_speckle: u32,

    /// Corner threshold (0-180, lower = sharper corners)
    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(i32).range(0..=180))]
    pub corner_threshold: i32,

    /// Layer difference (1-255, lower = more color layers)
    #[arg(long, default_value_t = 5, value_parser = clap::value_parser!(i32).range(1..=255))]
    pub layer_difference: i32,

    /// Length threshold (0.0-10.0, lower = more detail)
    #[arg(long, default_value_t = 1.0)]
    pub length_threshold: f64,

    /// Splice threshold (0-180, lower = preserve more details)
    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(i32).range(0..=180))]
    pub splice_threshold: i32,

    /// Max iterations (1-100, higher = better quality)
    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u32).range(1..=100))]
    pub max_iterations: u32,

    /// Path simplify mode
    #[arg(long, value_enum, default_value = "none")]
    pub mode: Mode,
}

impl Cli {
    pub fn get_output_path(&self) -> PathBuf {
        match &self.output {
            Some(path) => path.clone(),
            None => self.input.with_extension("svg"),
        }
    }
}

fn validate_image_extension(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    let allowed_extensions = ["jpg", "jpeg", "png", "webp"];
    let ext = s
        .rsplit('.')
        .next()
        .ok_or_else(|| format!("😒 {}", "File must have an extension".red().bold()))?
        .to_lowercase();
    if allowed_extensions.contains(&ext.as_str()) {
        Ok(path)
    } else {
        Err(format!(
            "😒 {}",
            format!(
                "Invalid file extension: {}. Allowed extensions are: {:?}",
                ext, allowed_extensions
            )
            .red()
            .bold()
        ))
    }
}
fn validate_svg_extension(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    let ext = s
        .rsplit('.')
        .next()
        .ok_or_else(|| format!("😒 {}", "File must have an extension".red().bold()))?
        .to_lowercase();
    if ext == "svg" {
        Ok(path)
    } else {
        Err(format!(
            "😒 {}",
            format!("Output file must have .svg extension, got .{}", ext)
                .red()
                .bold()
        ))
    }
}
