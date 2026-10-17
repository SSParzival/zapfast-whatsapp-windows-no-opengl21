//! An opaque header keeps scrolled text readable without a GL paint callback.

pub fn shape(rect: egui::Rect, colour: egui::Color32) -> egui::Shape {
    egui::Shape::rect_filled(rect, 0.0, colour.to_opaque())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translucent_themes_still_cover_the_text_under_the_header() {
        let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(300.0, 64.0));
        let colour = egui::Color32::from_rgba_unmultiplied(20, 30, 40, 60);
        let egui::Shape::Rect(header) = shape(rect, colour) else {
            panic!("the header must be a native egui shape, not a GPU callback");
        };
        assert_eq!(header.rect, rect);
        assert_eq!(header.fill.a(), 255);
        assert!(
            header.fill.r() > colour.r(),
            "unpremultiply the theme's colour"
        );
    }
}
