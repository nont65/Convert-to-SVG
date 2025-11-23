use anyhow::{Result, bail};
use clap::Parser;
use colored::*;
use vtracer::{Config, convert_image_to_svg};

use convert_to_svg::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.input.exists() {
        bail!(
            "😒 {} {:?}",
            "Input file does not exist:".red().bold(),
            cli.input
        );
    }

    let output_path = cli.get_output_path();

    println!(
        "😁 {} {}",
        "Loading image:".cyan().bold(),
        cli.input.display().to_string().yellow()
    );

    let config = Config {
        color_mode: cli.color_mode.to_vtracer_color_mode(),
        hierarchical: vtracer::Hierarchical::Stacked,
        filter_speckle: cli.filter_speckle as usize,
        color_precision: cli.color_precision,
        layer_difference: cli.layer_difference,
        mode: cli.mode.to_path_simplify_mode(),
        corner_threshold: cli.corner_threshold,
        length_threshold: cli.length_threshold,
        max_iterations: cli.max_iterations as usize,
        splice_threshold: cli.splice_threshold,
        path_precision: Some(cli.path_precision),
    };

    println!("⏳ {}", "Converting to SVG...".cyan().bold());
    convert_image_to_svg(&cli.input, &output_path, config).map_err(|e| anyhow::anyhow!(e))?;

    println!(
        "✅ {} {}",
        "Successfully saved to:".green().bold(),
        output_path.display().to_string().yellow()
    );

    Ok(())
}
