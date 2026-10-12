use eframe::egui;
use egui_dock::DockArea;

pub struct VegaApp {
    dock_tree: egui_dock::DockTree,
}

impl VegaApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure custom fonts here if needed
        // cc.egui_ctx.set_fonts(|ctx| { ... });

        let dock_tree = egui_dock::DockTree::new(vec![
            egui_dock::Node::leaf(egui_dock::Leaf::new(
                egui_dock::TabId::from(egui_dock::TabId::new(egui_dock::TabId::from(0))),
            )),
        ]);

        Self { dock_tree }
    }
}

impl eframe::App for VegaApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        // TODO: Create new document
                    }
                    if ui.button("Open").clicked() {
                        // TODO: Open file dialog
                    }
                    if ui.button("Save").clicked() {
                        // TODO: Save document
                    }
                    if ui.button("Export").clicked() {
                        // TODO: Export dialog
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        // TODO: Undo
                    }
                    if ui.button("Redo").clicked() {
                        // TODO: Redo
                    }
                });

                ui.menu_button("Image", |ui| {
                    if ui.button("Adjustments").clicked() {
                        // TODO: Open adjustments panel
                    }
                    if ui.button("Filters").clicked() {
                        // TODO: Open filters panel
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Zoom In").clicked() {
                        // TODO: Zoom in
                    }
                    if ui.button("Zoom Out").clicked() {
                        // TODO: Zoom out
                    }
                    if ui.button("Fit to Screen").clicked() {
                        // TODO: Fit to screen
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            DockArea::new(&mut self.dock_tree)
                .show_add_buttons(true)
                .show(ctx, &mut TabViewer {});
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Zoom: 100%");
                ui.label("Canvas: 1920x1080");
                ui.label("Memory: 256 MB");
            });
        });
    }
}

struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title.clone()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui);
    }
}

pub struct Tab {
    title: egui::WidgetText,
}

impl Tab {
    pub fn new(title: impl Into<egui::WidgetText>) -> Self {
        Self {
            title: title.into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Tab content here");
    }
}
