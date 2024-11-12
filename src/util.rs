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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_dimensions_nop() {
        assert_eq!(fit_dimensions(9, 16, 20, 20), (9, 16));
    }

    #[test]
    fn fit_dimensions_shrink_width() {
        assert_eq!(fit_dimensions(100, 200, 80, 200), (80, 160));
    }

    #[test]
    fn fit_dimensions_shrink_height() {
        assert_eq!(fit_dimensions(60, 90, 80, 60), (40, 60))
    }

    #[test]
    fn fit_dimensions_shrink_both() {
        assert_eq!(fit_dimensions(150, 200, 135, 144), (108, 144));
    }
}
