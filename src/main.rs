use std::{fs, process::ExitCode};

use asciificate::{util, AsciificateError};
use clap::Parser;
use image::{imageops::FilterType, ImageReader, LumaA};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    filename: String,

    #[arg(short, long, default_value_t = String::from(" .:-=+*#%@"))]
    scale: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    invert: bool,
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
    if args.scale.is_empty() {
        return Err(Box::new(AsciificateError::EmptyGrayScale));
    }

    let (term_width, term_height) = term_size::dimensions()
        .map(|(w, h)| (w as u32, h as u32))
        .unwrap_or((80, 40));

    let image = ImageReader::open(args.filename)?.decode()?;

    let (width, height) =
        util::fit_dimensions(image.width(), image.height(), term_width / 2, term_height);
    let width = 2 * width;

    let grayscale = args.scale.chars().collect::<Vec<_>>();

    let result = image
        .resize_exact(width as u32, height as u32, FilterType::Gaussian)
        .into_luma_alpha8()
        .pixels()
        .map(|&LumaA([value, alpha])| {
            let mut value = value as f32 * (alpha as f32 / 255.0);

            if args.invert {
                value = 255.0 - value;
            }

            grayscale[(value * (grayscale.len() as f32 / 256.0)) as usize]
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
