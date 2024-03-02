/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct JohnRs {
    show_sidebar: bool,
}

impl Default for JohnRs {
    fn default() -> Self {
        Self { show_sidebar: true }
    }
}

impl JohnRs {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for JohnRs {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::widgets::global_dark_light_mode_buttons(ui);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Max), |ui| {
                ui.toggle_value(&mut self.show_sidebar, "Sidebar");
            });
        });

        egui::SidePanel::right("side_panel")
            .resizable(false)
            .show_animated(ctx, true, |ui| {
                ui.label("Hello!");
                ui.label("world!");
            });

        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}
