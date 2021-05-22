mod rect;
mod wgpu_painter;

use futures::executor::block_on;
use painting::{Color, RRect, Rect};
use rect::RectPainter;
use wgpu_painter::WgpuPainter;

pub type Bitmap = Vec<u8>;

pub struct Painter {
    backend: WgpuPainter,
    rect_painter: RectPainter,
}

impl Painter {
    pub async fn new() -> Self {
        let backend = WgpuPainter::new().await;

        let rect_painter = RectPainter::new();

        Self {
            backend,
            rect_painter,
        }
    }

    pub async fn paint(&mut self, size: (u32, u32)) -> Option<Bitmap> {
        let device = self.backend.device();
        let data = [self.rect_painter.get_paint_data(device, size)];

        let buffer = self.backend.paint(size, &data).await;
        self.backend.output(size, buffer).await
    }
}

impl painting::Painter for Painter {
    fn fill_rect(&mut self, rect: &Rect, color: &Color) {
        self.rect_painter.draw_solid_rect(rect, color);
    }

    fn fill_rrect(&mut self, rect: &RRect, color: &Color) {
        self.rect_painter.draw_solid_rrect(rect, color);
    }
}