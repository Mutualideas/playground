use eframe::egui::*;//glob import

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(vec2(480.0, 270.0))
            .with_min_inner_size(vec2(480.0, 270.0))
            .with_transparent(true),
        ..Default::default()
    };

    let _ = eframe::run_native(
        &format!("My App{}", env!("testapp")),
        native_options,
        Box::new(|cc| {
            let mut visuals = Visuals::dark();
            // make panels transparent
            visuals.panel_fill = Color32::from_rgba_premultiplied(
                visuals.panel_fill.r(),
                visuals.panel_fill.g(),
                visuals.panel_fill.b(),
                100,
            );
            cc.egui_ctx.set_visuals(visuals);
            cc.egui_ctx
                .set_pixels_per_point(cc.egui_ctx.native_pixels_per_point().unwrap_or(1.0) * 1.2);
            Box::new(App::default())
        }),
    );
}

#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("main viewport");
        });

        ctx.show_viewport_deferred(
            ViewportId("my viewport".into()),
            ViewportBuilder::default()
                .with_inner_size(vec2(480.0, 270.0))
                .with_min_inner_size(vec2(480.0, 270.0))
                .with_transparent(true),
            |ctx, _| {
                CentralPanel::default().show(ctx, |ui| {
                    ui.label("deferred viewport");
                });
            },
        )
    }

    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> [f32; 4] {
        // fully transparent clear
        Default::default()
    }
}
