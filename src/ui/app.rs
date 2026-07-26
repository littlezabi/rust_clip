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
        // Position the "×" button at the top-right of header_rect
          let close_btn_rect = egui::Rect::from_min_size(
              header_rect.right_top() + egui::vec2(-32.0, 5.0),
              egui::vec2(24.0, 24.0),
          );
          let close_response = ui.put(
              close_btn_rect,
              egui::Button::new(
                  egui::RichText::new("×")
                      .size(20.0)
                      .color(egui::Color32::GRAY),
              )
              .frame(false),
          );
          // Hover & click handlers for close button
          if close_response.hovered() {
              ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
          }
          if close_response.clicked() {
              ctx.send_viewport_cmd(egui::ViewportCommand::Close);
          }
          ui.add_space(4.0);


        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_query);
        });
        ui.add_space(28.0);
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
