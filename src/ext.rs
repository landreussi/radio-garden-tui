use ratatui::layout::Rect;

pub(super) trait Split {
    fn split_vertically(&self, splits: u16) -> Vec<Rect>;
}

impl Split for Rect {
    fn split_vertically(&self, splits: u16) -> Vec<Rect> {
        let width = self.width / splits;
        let height = self.height;
        let mut x = self.x;
        let y = self.y;
        let mut rectangles = Vec::with_capacity(splits as usize);

        for _ in 0..splits {
            let rect = Rect::new(x, y, width, height);
            rectangles.push(rect);
            x += width;
        }

        rectangles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_vertically_rect_in_two() {
        // Arrange
        let rect = Rect::new(0, 0, 50, 50);
        let expected_rects = vec![Rect::new(0, 0, 25, 50), Rect::new(25, 0, 25, 50)];

        // Act
        let returned_rects = rect.split_vertically(2);

        // Assert
        assert_eq!(expected_rects, returned_rects);
    }
}
