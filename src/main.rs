use asciificate::{
    cli::{Cli, FillOption},
    convert::{self},
};
use image::{imageops::FilterType, ImageReader};
use std::{fs, process::ExitCode};

fn main_unwrapped(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(FillOption::Quarter) = cli.fill {
        if cli.color.is_none() {
            return Err("quarter fill requires a color mode".into());
        }
    }

    let image = ImageReader::open(&cli.filename)?.decode()?;

    let (target_width, target_height) = cli.fit.or_else(term_size::dimensions).unwrap_or((80, 40));

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
