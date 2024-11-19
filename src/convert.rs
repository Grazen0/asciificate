use crate::color;
use clap::ValueEnum;
use colored::{Color, Colorize};
use image::Rgb;

#[derive(Debug, Clone, ValueEnum)]
pub enum ColorOption {
    Ansi,
    TrueColor,
}

fn rgb_to_color(r: u8, g: u8, b: u8, opt: &ColorOption) -> Color {
    match opt {
        ColorOption::Ansi => color::rgb_to_ansi(r, g, b),
        ColorOption::TrueColor => Color::TrueColor { r, g, b },
    }
}

pub fn quarter(pixel_matrix: Vec<&[&Rgb<u8>]>, color_option: &ColorOption) -> Vec<String> {
    pixel_matrix
        .chunks(2)
        .map(|row_pair| {
            row_pair[0]
                .into_iter()
                .zip(row_pair[1])
                .map(|(&&Rgb([r1, g1, b1]), &&Rgb([r2, g2, b2]))| {
                    let color_top = rgb_to_color(r1, g1, b1, color_option);
                    let color_bottom = rgb_to_color(r2, g2, b2, color_option);

                    '▀'
                        .to_string()
                        .color(color_top)
                        .on_color(color_bottom)
                        .to_string()
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

pub fn half(
    pixel_matrix: Vec<&[&Rgb<u8>]>,
    color_option: &Option<ColorOption>,
    invert: bool,
) -> Vec<String> {
    pixel_matrix
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|&&Rgb([r, g, b])| match color_option {
                    Some(color_option) => {
                        let color = rgb_to_color(r, g, b, color_option);
                        " ".to_string().on_color(color).to_string()
                    }
                    None => {
                        const FILL_SCALE: [char; 5] = [' ', '░', '▒', '▓', '█'];
                        let mut value = color::rgb_to_luma(r, g, b);

                        if invert {
                            value = 255.0 - value;
                        }

                        crate::index_by_percent(&FILL_SCALE, value / 256.0).to_string()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

pub fn regular(
    pixel_matrix: Vec<&[&Rgb<u8>]>,
    gray_scale: &Vec<char>,
    color_option: &Option<ColorOption>,
    invert: bool,
) -> Vec<String> {
    pixel_matrix
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|&&Rgb([r, g, b])| {
                    let mut value = color::rgb_to_luma(r, g, b);

                    if invert {
                        value = 255.0 - value;
                    }

                    let result = crate::index_by_percent(&gray_scale, value / 256.0).to_string();

                    match color_option {
                        Some(color_option) => {
                            let color = rgb_to_color(r, g, b, color_option);
                            result.color(color).to_string()
                        }
                        None => result,
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}
