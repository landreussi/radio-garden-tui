use tui::layout::Rect;

pub(super) trait Split {
    fn split_horizontally(&self, splits: u16) -> Vec<Rect>;
}

impl Split for Rect {
    fn split_horizontally(&self, splits: u16) -> Vec<Rect> {
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
