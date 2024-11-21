use colored::Color;

pub fn rgb_to_ansi(r: u8, g: u8, b: u8) -> Color {
    let rgb = [r, g, b].map(i32::from);

    const ANSI_COLORS: [([u8; 3], Color); 16] = [
        ([0, 0, 0], Color::Black),
        ([128, 0, 0], Color::Red),
        ([0, 128, 0], Color::Green),
        ([128, 128, 0], Color::Yellow),
        ([0, 0, 128], Color::Blue),
        ([128, 0, 128], Color::Magenta),
        ([0, 128, 128], Color::Cyan),
        ([192, 192, 192], Color::White),
        ([128, 128, 128], Color::BrightBlack),
        ([255, 0, 0], Color::BrightRed),
        ([0, 255, 0], Color::BrightGreen),
        ([255, 255, 0], Color::BrightYellow),
        ([0, 0, 255], Color::BrightBlue),
        ([255, 0, 255], Color::BrightMagenta),
        ([0, 255, 255], Color::BrightCyan),
        ([255, 255, 255], Color::BrightWhite),
    ];

    ANSI_COLORS
        .iter()
        .min_by_key(|(color_rgb, _)| {
            color_rgb
                .map(i32::from)
                .into_iter()
                .zip(&rgb)
                .map(|(m, n)| (m - n).pow(2))
                .sum::<i32>()
        })
        .unwrap()
        .1
}

pub fn rgb_to_luma(r: u8, g: u8, b: u8) -> f32 {
    0.2126 * (r as f32) + 0.7152 * (g as f32) + 0.0722 * (b as f32)
}
