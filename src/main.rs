use std::{fs, process::ExitCode};

use asciificate::{util, AsciificateError};
use clap::Parser;
use image::{imageops::FilterType, ImageReader, Rgba};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    filename: String,

    #[arg(short, long, default_value_t = String::from(" .:-=+*#%@"))]
    gray_scale: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let result = main_unwrapped(args);

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn main_unwrapped(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.gray_scale.is_empty() {
        return Err(Box::new(AsciificateError::EmptyGrayScale));
    }

    let (term_width, term_height) = term_size::dimensions()
        .map(|(w, h)| (w as u32, h as u32))
        .unwrap_or((80, 40));

    let image = ImageReader::open(args.filename)?.decode()?;

    let (width, height) =
        util::fit_dimensions(image.width(), image.height(), term_width / 2, term_height);
    let width = 2 * width;

    let grayscale = args.gray_scale.chars().collect::<Vec<_>>();

    let result = image
        .resize_exact(width as u32, height as u32, FilterType::Gaussian)
        .into_rgba8()
        .pixels()
        .map(|Rgba(data)| {
            let [r, g, b, a] = data.map(|n| n as f32);
            let avg = (r + g + b + a) / 4.0;
            grayscale[(avg * grayscale.len() as f32 / 256.0) as usize]
        })
        .collect::<Vec<_>>()
        .chunks_exact(width as usize)
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    match args.output {
        Some(output_path) => fs::write(output_path, result)?,
        None => println!("{}", result),
    }

    Ok(())
}
