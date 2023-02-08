use std::sync::Arc;
use eframe::egui;
use eframe::{EguiSkiaPaintCallback, skia_safe::{Paint, self, font_style::{Weight, Width, Slant}, Color4f}};

pub struct CustomSkia {

}

impl CustomSkia {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        egui::Window::new("Draw to skia").show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());
                ui.painter().add(egui::PaintCallback {
                    rect: rect.clone(),
                    callback: Arc::new(EguiSkiaPaintCallback::new(move |canvas, physical_rect, pixels_per_point| {
                        let mut paint = Paint::default();
                        
                        paint.set_color4f(Color4f::new(0.8, 0.8, 0.8, 1.), None);
                        canvas.draw_rect(physical_rect, &paint);
                        canvas.translate((physical_rect.left, physical_rect.top));
    
                        let typeface = skia_safe::Typeface::default();
                        let font = skia_safe::Font::new(typeface.clone(), 20.0 * pixels_per_point);

                        paint.set_color4f(Color4f::new(1., 0., 0., 1.), None);
                        let blob = skia_safe::TextBlob::from_str("This is Skia drawing text", &font).unwrap();
                        canvas.draw_text_blob(&blob, (10. * pixels_per_point, 50. * pixels_per_point), &paint);

                        paint.set_color4f(Color4f::new(0., 1., 0., 1.), None);
                        let blob = skia_safe::TextBlob::from_str("if OpenGL mode init failed", &font).unwrap();
                        canvas.draw_text_blob(&blob, (10. * pixels_per_point, 100. * pixels_per_point), &paint);

                        paint.set_color4f(Color4f::new(0., 0., 1., 1.), None);
                        let blob = skia_safe::TextBlob::from_str("it will fallcack to CPU mode", &font).unwrap();
                        canvas.draw_text_blob(&blob, (10. * pixels_per_point, 150. * pixels_per_point), &paint);

                        paint.set_color4f(Color4f::new(0., 0., 0., 1.), None);
                        let font = skia_safe::Font::new(typeface, 15.0 * pixels_per_point);
                        let blob = skia_safe::TextBlob::from_str("by @RustyTsuki", &font).unwrap();
                        canvas.draw_text_blob(&blob, (10. * pixels_per_point, 200. * pixels_per_point), &paint);
                    })),
                })
            });
        });
    }
}
