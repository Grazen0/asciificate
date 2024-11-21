pub mod cli;
pub mod color;
pub mod convert;

pub fn index_by_percent<T>(arr: &[T], position: f32) -> &T {
    &arr[(position * arr.len() as f32) as usize]
}

pub fn fit_dimensions(
    mut width: usize,
    mut height: usize,
    target_width: usize,
    target_height: usize,
) -> (usize, usize) {
    if width > target_width {
        height = (height as f32 * (target_width as f32 / width as f32)) as usize;
        width = target_width;
    }

    if height > target_height {
        width = (width as f32 * (target_height as f32 / height as f32)) as usize;
        height = target_height;
    }

    (width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_by_percent_zero() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(index_by_percent(&arr, 0.0), &1);
    }

    #[test]
    fn index_by_percent_quarter() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(index_by_percent(&arr, 0.25), &2);
    }

    #[test]
    fn index_by_percent_half() {
        let arr = [42, 36, 7];
        assert_eq!(index_by_percent(&arr, 0.5), &36);
    }

    #[test]
    #[should_panic]
    fn index_by_percent_one() {
        let arr = [1, 2, 3, 4, 5];
        index_by_percent(&arr, 1.0);
    }

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
