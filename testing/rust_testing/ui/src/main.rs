use eframe::{NativeOptions, egui::CentralPanel, egui::Slider, epi::App, run_native};

struct RuhigeWaldgeraeusche;

impl App for RuhigeWaldgeraeusche {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        let mut slider_value: f32 = 0.0;
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello Sailor");
            if ui.button("click me").clicked() { print!("MOIN\n"); }
            ui.add(Slider::new(&mut slider_value, 0.0..=100.0));
            ui.label(format!("Slider value: {}", slider_value).as_str());
        });
    }

    fn name(&self) -> &str {
        "Ruhige Waldgeräusche"
    }
}

fn main() {
    let app: RuhigeWaldgeraeusche = RuhigeWaldgeraeusche;
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
}