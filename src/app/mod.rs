mod custom_skia;
use eframe::egui;
use egui_demo_lib;

pub struct SkiaApp {
    demo_windows: egui_demo_lib::DemoWindows,
    custom_skia: custom_skia::CustomSkia,
}

impl SkiaApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_styles(&cc.egui_ctx);

        Self {
            demo_windows: egui_demo_lib::DemoWindows::default(),
            custom_skia: custom_skia::CustomSkia::new(),
        }
    }
}

impl eframe::App for SkiaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_windows.ui(ctx);
        self.custom_skia.update(ctx);
    }
}

fn setup_custom_styles(ctx: &egui::Context) {
    let style = egui::Style {
        visuals: egui::Visuals::light(),
        animation_time: 0., // cpu mode is slower, so we disable animation
        ..egui::Style::default()
    };
    ctx.set_style(style);
}
