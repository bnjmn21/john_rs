use egui::Ui;

pub struct CollapsibleModule<T: GuiModule> {
    collapsed: bool,
    module: T,
}

impl<T: GuiModule> CollapsibleModule<T> {
    pub fn new(collapsed: bool) -> Self {
        Self {
            collapsed,
            module: Default::default(),
        }
    }
}

pub trait GuiModule: Default {
    fn render(&mut self, ui: &mut Ui);
    fn name(&self) -> String;
}

#[derive(Default)]
pub struct Readme;

impl GuiModule for Readme {
    fn render(&mut self, ui: &mut Ui) {
        ui.label("Hello, world!");
    }

    fn name(&self) -> String {
        "Read me".into()
    }
}
