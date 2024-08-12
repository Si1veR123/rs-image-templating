#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize
}

impl Rect {
    /// Returns false if `width+x` or `height+y` cannot fit in a `usize`
    pub fn contains(&self, x: usize, y: usize) -> bool {
        let upper_x = match self.x.checked_add(self.width) {
            Some(x) => x,
            None => return false
        };
        let upper_y = match self.y.checked_add(self.height) {
            Some(y) => y,
            None => return false
        };

        x >= self.x && x < upper_x && y >= self.y && y < upper_y
    }
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    type ContainsTestCases<'a> = [(Rect, &'a [(usize, usize, bool)])];
    fn run_contains_test(contains: &ContainsTestCases) {
        for rect in contains {
            for case in rect.1 {
                assert_eq!(rect.0.contains(case.0, case.1), case.2)
            }
        }
    }

    #[test]
    fn zeroed_coordinate() {
        let rect_test_cases = [
            (
                Rect::default(),
                [
                    (0, 0, false), (0, 1, false), (1, 0, false),
                    (100, 0, false), (0, 100, false), (50, 50, false),
                    (usize::MAX, usize::MAX, false)
                ].as_slice()
            ),
            (
                Rect { x: 0, y: 0, width: 100, height: 100 },
                [
                    (0, 0, true), (0, 100, false), (0, 99, true),
                    (100, 0, false), (99, 0, true), (50, 50, true),
                    (150, 150, false), (50, 150, false), (150, 50, false),
                    (100, 100, false)
                ].as_slice()
            ),
            (
                Rect { x: 0, y: 0, width: 100, height: 50 },
                [
                    (0, 0, true), (0, 100, false), (0, 99, false),
                    (100, 0, false), (99, 0, true), (50, 50, false),
                    (150, 150, false), (50, 150, false), (150, 50, false),
                    (100, 100, false), (25, 25, true)
                ].as_slice()
            ),
            (
                Rect { x: 0, y: 0, width: usize::MAX, height: usize::MAX },
                [
                    (0, 0, true), (1000, 1000, true), (usize::MAX, usize::MAX, false)
                ].as_slice()
            )
        ];
        
        run_contains_test(&rect_test_cases);
    }

    #[test]
    fn varied_coordinate() {
        let rect_test_cases = [
            (
                Rect { x: 50, y: 50, width: 0, height: 0 },
                [
                    (0, 0, false), (0, 1, false), (1, 0, false),
                    (100, 0, false), (0, 100, false), (50, 50, false),
                    (usize::MAX, usize::MAX, false)
                ].as_slice()
            ),
            (
                Rect { x: 50, y: 50, width: 100, height: 100 },
                [
                    (0, 0, false), (0, 100, false), (0, 99, false),
                    (100, 0, false), (99, 0, false), (50, 50, true),
                    (150, 150, false), (50, 150, false), (150, 50, false),
                    (149, 149, true), (100, 100, true), (200, 200, false)
                ].as_slice()
            ),
            (
                Rect { x: 200, y: 100, width: 65535, height: 10 },
                [
                    (0, 0, false), (1000, 1000, false), (200, 100, true),
                    (50000, 105, true), (65635, 109, true), (205, 50, false),
                    (1000, 150, false)
                ].as_slice()
            )
        ];
        
        run_contains_test(&rect_test_cases);
    }

    #[test]
    fn overflow() {
        let rect_test_cases = [
            (
                Rect { x: 1, y: 1, width: usize::MAX, height: usize::MAX },
                [
                    (0, 0, false), (0, 1, false), (1, 1, false),
                    (100, 0, false), (0, 100, false), (50, 50, false),
                    (usize::MAX, usize::MAX, false)
                ].as_slice()
            ),
            (
                Rect { x: usize::MAX, y: usize::MAX, width: usize::MAX, height: usize::MAX },
                [
                    (0, 0, false), (100, 0, false), (0, 100, false),
                    (50, 50, false), (usize::MAX, usize::MAX, false)
                ].as_slice()
            )
        ];
        
        run_contains_test(&rect_test_cases);
    }
}
