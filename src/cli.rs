use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
pub enum ColorOption {
    Ansi,
    TrueColor,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum FillOption {
    /// One color per character
    Half,
    /// Two colors per character
    Quarter,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Image source
    pub filename: PathBuf,

    /// ASCII gray scale to use
    #[arg(short, long, value_parser = scale_parser, default_value = " .:-=+*#%@")]
    pub scale: std::vec::Vec<char>, // Need `std::vec::` so that clap takes the type literally

    /// Dimensions to fit the image to
    #[arg(short, long, value_parser = size_parser)]
    pub fit: Option<(usize, usize)>,

    /// A file to redirect the output to
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Invert the image colors (useful for terminals with white background)
    #[arg(short, long)]
    pub invert: bool,

    /// Color mode to use (in bits)
    #[arg(short, long)]
    pub color: Option<ColorOption>,

    /// Uses solid pixels instead of the ASCII gray scale
    #[arg(short = 'F', long)]
    pub fill: Option<FillOption>,
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
