use crate::models::ClipboardItem;
use eframe::egui;
use std::sync::mpsc::Receiver;

pub struct ClipboardApp {
    rx: Receiver<ClipboardItem>,
    history: Vec<ClipboardItem>,
    search_query: String,
}

impl ClipboardApp {
    fn get_app_frame() -> egui::Frame {
        let panel_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 240);

        // Define a smooth drop shadow
        let custom_shadow = egui::Shadow {
            offset: [0, 4],
            blur: 20,
            spread: 2,
            color: egui::Color32::from_black_alpha(120), // Transparency of the shadow
        };

        egui::Frame::new()
            .fill(panel_fill)
            .corner_radius(12.0)
            .shadow(custom_shadow)
            .inner_margin(6.0)
    }

    pub fn new(rx: Receiver<ClipboardItem>) -> Self {
        Self {
            rx,
            history: Vec::new(),
            search_query: String::new(),
        }
    }

    pub fn run(rx: Receiver<ClipboardItem>) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([380.0, 550.0])
                .with_min_inner_size([380.0, 550.0])
                .with_max_inner_size([380.0, 550.0])
                .with_decorations(false)
                .with_transparent(true)
                .with_window_level(egui::WindowLevel::AlwaysOnTop),
            ..Default::default()
        };

        eframe::run_native(
            "RustClip",
            options,
            Box::new(move |_cc| Ok(Box::new(Self::new(rx)))),
        )
    }

    fn render_title_and_clear_button(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let header_rect = ui.allocate_space(egui::vec2(ui.available_width(), 35.0)).1;
        ui.painter().text(
            header_rect.left_top() + egui::vec2(12.0, 8.0),
            egui::Align2::LEFT_TOP,
            "Clipboard",
            egui::FontId::proportional(16.0),
            egui::Color32::WHITE,
        );

        let clear_btn = header_rect.right_top() + egui::vec2(-35.0, 12.0);

        let clear_response = ui.interact(
            egui::Rect::from_center_size(clear_btn, egui::vec2(24.0, 24.0)),
            ui.id().with("clear_button"),
            egui::Sense::click(),
        );

        let is_hovered = clear_response.hovered();
        let how_hovered = ui.ctx().animate_bool(ui.id().with("clear_anim"), is_hovered);

        // Color transition: Light White -> Full Bright White on hover
        let clear_text_color = egui::Color32::from_gray(180).lerp_to_gamma(
            egui::Color32::WHITE,
            how_hovered,
        );

        ui.painter().text(
            clear_btn,
            egui::Align2::CENTER_CENTER,
            "Clear all",
            egui::FontId::proportional(14.0),
            clear_text_color,
        );

        if is_hovered {
            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        if clear_response.clicked() {
            self.history.clear();
        }
        ui.add_space(28.0);
    }

    fn render_header(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let header_rect = ui.allocate_space(egui::vec2(ui.available_width(), 35.0)).1;

        let header_response = ui.interact(
            header_rect,
            ui.id().with("drag_handle"),
            egui::Sense::drag(),
        );

        if header_response.dragged() {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }

        ui.painter().text(
            header_rect.left_top() + egui::vec2(12.0, 8.0),
            egui::Align2::LEFT_TOP,
            "📋 RustClip",
            egui::FontId::proportional(16.0),
            egui::Color32::WHITE,
        );

        // Cross ("✕") Button Position (Right)
        let cross_pos = header_rect.right_top() + egui::vec2(-16.0, 12.0);

        let cross_response = ui.interact(
            egui::Rect::from_center_size(cross_pos, egui::vec2(24.0, 24.0)),
            ui.id().with("cross_close"),
            egui::Sense::click(),
        );

        let is_hovered = cross_response.hovered();
        let how_hovered = ui.ctx().animate_bool(ui.id().with("cross_anim"), is_hovered);

        // Color transition: Light White -> Full Bright White on hover
        let icon_color = egui::Color32::from_gray(180).lerp_to_gamma(
            egui::Color32::WHITE,
            how_hovered,
        );

        ui.painter().text(
            cross_pos,
            egui::Align2::CENTER_CENTER,
            "×",
            egui::FontId::proportional(20.0),
            icon_color,
        );

        if is_hovered {
            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        if cross_response.clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
    
    fn render_search(&mut self, ui: &mut egui::Ui, ctx: &egui::Context){
        
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_query);
        });
        
    }
}

impl eframe::App for ClipboardApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain all items currently in the channel buffer instantly
        while let Ok(item) = self.rx.try_recv() {
            self.history.insert(0, item);
        }
        // app_frame configuration in get_app_frame where window design is exists.
        let app_frame = Self::get_app_frame();
        egui::CentralPanel::default()
            .frame(app_frame)
            .show(ctx, |ui| {
                
                // Inside your UI update loop:
                self.render_header(ui, ctx);
                self.render_search(ui, ctx);
                ui.add_space(20.0);
                self.render_title_and_clear_button(ui, ctx);
                

                crate::ui::components::render_history_list(
                    ui,
                    &self.history,
                    &mut |clicked_item| {
                        match clicked_item {
                            ClipboardItem::Text(text) => {
                                println!("Item clicked, ready to copy back: {}", text);
                                // TODO: Add your logic here to re-send text to system clipboard
                            }
                            _ => {}
                        }
                    },
                );
                
            });
    }
}
