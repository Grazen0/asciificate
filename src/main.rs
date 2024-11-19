use asciificate::convert::{self, ColorOption};
use clap::{Parser, ValueEnum};
use image::{imageops::FilterType, ImageReader};
use std::{fs, path::PathBuf, process::ExitCode};

#[derive(Debug, Clone, ValueEnum)]
enum FillOption {
    /// One color per character
    Half,
    /// Two colors per character
    Quarter,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Image source
    filename: PathBuf,

    /// ASCII gray scale to use
    #[arg(short, long, value_parser = scale_parser, default_value = " .:-=+*#%@")]
    scale: std::vec::Vec<char>, // Need `std::vec::` so that clap takes the type literally

    /// Dimensions to fit the image to
    #[arg(short, long, value_parser = size_parser)]
    fit: Option<(usize, usize)>,

    /// A file to redirect the output to
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Invert the image colors (useful for terminals with white background)
    #[arg(short, long)]
    invert: bool,

    /// Color mode to use (in bits)
    #[arg(short, long)]
    color: Option<ColorOption>,

    /// Uses solid pixels instead of the ASCII gray scale
    #[arg(short = 'F', long)]
    fill: Option<FillOption>,
}

fn size_parser(s: &str) -> Result<(usize, usize), String> {
    let (w, h) = s.split_once('x').ok_or("invalid dimensions format")?;

    Ok((
        w.parse::<usize>().map_err(|e| e.to_string())?,
        h.parse::<usize>().map_err(|e| e.to_string())?,
    ))
}

fn scale_parser(s: &str) -> Result<Vec<char>, String> {
    if s.is_empty() {
        Err("gray scale should not be empty".to_string())
    } else {
        Ok(s.chars().collect())
    }
}

fn main_unwrapped(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(FillOption::Quarter) = cli.fill {
        if cli.color.is_none() {
            return Err("quarter fill requires a color mode".into());
        }
    }

    let image = ImageReader::open(&cli.filename)?.decode()?;

    let (target_width, target_height) = cli
        .fit
        .or_else(|| term_size::dimensions().map(|(w, h)| (w, h)))
        .unwrap_or((80, 40));

    let (width, height) = if let Some(FillOption::Quarter) = cli.fill {
        asciificate::fit_dimensions(
            image.width() as usize,
            image.height() as usize,
            target_width,
            target_height * 2,
        )
    } else {
        let (width, height) = asciificate::fit_dimensions(
            image.width() as usize,
            image.height() as usize,
            target_width / 2,
            target_height,
        );
        (2 * width, height)
    };

    let rgb = image
        .resize_exact(width as u32, height as u32, FilterType::Gaussian)
        .into_rgb8();

    let pixels = rgb.pixels().collect::<Vec<_>>();
    let pixel_matrix = pixels.chunks_exact(width).collect::<Vec<_>>();

    let lines = match cli.fill {
        Some(FillOption::Quarter) => convert::quarter(pixel_matrix, cli.color.as_ref().unwrap()),
        Some(FillOption::Half) => convert::half(pixel_matrix, &cli.color, cli.invert),
        None => convert::regular(pixel_matrix, &cli.scale, &cli.color, cli.invert),
    };

    let result = lines.join("\n");

    match &cli.output {
        Some(output_path) => fs::write(output_path, result)?,
        None => println!("{}", result),
    }

    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = main_unwrapped(&cli);

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
