use eframe::epi;
use eframe::egui;

#[derive(Default)]
pub struct App {

}

impl epi::App for App {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {

        });
    }

    fn name(&self) -> &str {
        "GGST BBScript Unpacker"
    }
}