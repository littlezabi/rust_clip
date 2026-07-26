use crate::models::ClipboardItem;
use eframe::egui;

/// Renders the scrollable history list and handles click-to-copy interactions.
pub fn render_history_list(
    ui: &mut egui::Ui,
    history: &[ClipboardItem],
    on_item_clicked: &mut impl FnMut(&ClipboardItem),
) {
    egui::ScrollArea::vertical()
        .id_salt("top_scroll_area")
        .show(ui, |ui| {
            for (id, item) in history.iter().enumerate() {
                if render_clipboard_card(ui, item, id).clicked() {
                    on_item_clicked(item);
                }
                ui.add_space(4.0);
            }
        });
}

fn render_clipboard_card(ui: &mut egui::Ui, item: &ClipboardItem, index: usize) -> egui::Response {
    let card_id = ui.make_persistent_id(format!("clipboard_card_{}", index));

    let is_hovered = ui
        .ctx()
        .read_response(card_id)
        .map_or(false, |r| r.hovered());
    let how_hovered = ui.ctx().animate_bool(card_id, is_hovered);

    // 1. Background: Dark -> Subtle White tint on hover
    let bg_color = egui::Color32::from_gray(24).lerp_to_gamma(
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 5),
        how_hovered,
    );

    // 2. Border: Dark gray -> Bright White on hover
    let border_color = egui::Color32::from_gray(40).lerp_to_gamma(
        egui::Color32::from_gray(80),
        how_hovered,
    );

    let frame = egui::Frame::group(ui.style())
        .fill(bg_color)
        .stroke(egui::Stroke::new(1.0, border_color))
        .corner_radius(6.0)
        .inner_margin(8.0);

    let response = frame
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            match item {
                ClipboardItem::Text(text) => {
                    render_text_clipboard_item(ui, text, index);
                }
                ClipboardItem::Image { mime, data } => {
                    ui.label(format!("🖼️ Image [{}] ({} bytes)", mime, data.len()));
                }
                ClipboardItem::Files(paths) => {
                    ui.label(format!("📁 Files ({} items)", paths.len()));
                }
            }
        })
        .response;

    // Register card_id with the exact bounding rect of the card
    let response = ui.interact(response.rect, card_id, egui::Sense::click());

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    response
}

fn render_text_clipboard_item(ui: &mut egui::Ui, text: &str, index: usize) {
    ui.horizontal(|ui| {
        
        let max_char = 117;
        let is_truncated = text.trim().chars().count() > max_char;
        let display_text: String = if is_truncated {
            format!(
                "{}...",
                text.trim().chars().take(max_char).collect::<String>()
            )
        } else {
            text.trim().to_string()
        };

        let styled_text = egui::RichText::new(display_text.clone())
            .monospace()
            .color(egui::Color32::LIGHT_GRAY);

        // DYNAMIC MEASUREMENT: Ask egui font engine for exact wrapped line count
        let font_id = ui
            .style()
            .text_styles
            .get(&egui::TextStyle::Monospace)
            .cloned()
            .unwrap_or_else(|| egui::FontId::monospace(12.0));

        let available_w = ui.available_width();
        let galley = ui.fonts(|f| {
            f.layout(
                display_text,
                font_id,
                egui::Color32::LIGHT_GRAY,
                available_w,
            )
        });

        // True if text fits on 1 line on the user's screen right now
        let is_single_line = galley.rows.len() <= 1;

        if is_single_line {
            // Render directly: Takes exact 1-line natural height
            ui.add(egui::Label::new(styled_text).wrap());
        } else {
            // Render in ScrollArea: Capped at 60.0 (3 lines max) with scrolling
            let preview_height = 60.0;
            egui::ScrollArea::vertical()
                .id_salt(format!("card_scroll_{}", index))
                .max_height(preview_height)
                .show(ui, |ui| {
                    ui.add(egui::Label::new(styled_text).wrap());
                });
        }
    });
}
