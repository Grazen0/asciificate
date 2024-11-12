pub fn fit_dimensions(
    mut width: u32,
    mut height: u32,
    target_width: u32,
    target_height: u32,
) -> (u32, u32) {
    if width > target_width {
        height = (height as f32 * (target_width as f32 / width as f32)) as u32;
        width = target_width;
    }

    if height > target_height {
        width = (width as f32 * (target_height as f32 / height as f32)) as u32;
        height = target_height;
    }

    (width, height)
}
