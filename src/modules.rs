use egui::{Context, ScrollArea, Ui, Window};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use serde::{de::DeserializeOwned, Serialize};

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Modules {
    pub readme: CollapsibleModule<Readme>,
}

impl Modules {
    pub fn sidebar(&mut self, ui: &mut Ui) {
        self.readme.sidebar_button(ui);
    }

    pub fn windows(&mut self, ctx: &Context, ui: &mut Ui) {
        self.readme.window(ctx, ui);
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct CollapsibleModule<T: GuiModule> {
    visible: bool,
    data: T::Data,
    #[serde(skip)]
    module: T,
}

impl<T: GuiModule> CollapsibleModule<T> {
    pub fn sidebar_button(&mut self, ui: &mut Ui) {
        ui.toggle_value(&mut self.visible, T::name());
    }

    pub fn window(&mut self, ctx: &Context, ui: &mut Ui) {
        let window = Window::new(T::name())
            .constrain_to(ui.max_rect())
            .collapsible(false)
            .open(&mut self.visible)
            .resizable(true);
        let window = T::window_settings(window);
        window.show(ctx, |ui| {
            self.module.render(ui);
        });
    }
}

impl<T: GuiModule> Default for CollapsibleModule<T> {
    fn default() -> Self {
        Self {
            visible: T::COLLAPSED_BY_DEFAULT,
            module: Default::default(),
            data: Default::default(),
        }
    }
}

pub trait GuiModule: Default {
    fn render(&mut self, ui: &mut Ui);
    fn name() -> String;
    fn window_settings(window: Window<'_>) -> Window<'_> {
        window
    }
    type Data: DeserializeOwned + Serialize + Default;
    const COLLAPSED_BY_DEFAULT: bool;
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Readme {
    text: Option<String>,
}

impl GuiModule for Readme {
    fn render(&mut self, ui: &mut Ui) {
        ScrollArea::both().show(ui, |ui| {
            let mut cache = CommonMarkCache::default();
            if self.text.is_none() {
                self.text = Some(
                    include_str!("../README.md")
                        .lines()
                        .filter(|line| !line.contains("!["))
                        .collect::<Vec<&str>>()
                        .join("\n"),
                );
            }
            CommonMarkViewer::new("viewer").show(ui, &mut cache, self.text.as_ref().unwrap());
        });
    }

    fn name() -> String {
        "Read Me".into()
    }

    fn window_settings(window: Window<'_>) -> Window<'_> {
        window
    }

    type Data = ();

    const COLLAPSED_BY_DEFAULT: bool = true;
}
