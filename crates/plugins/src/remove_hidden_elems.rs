use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Remove hidden elements (zero sized, with absent attributes).
///
/// Note: This is a simplified implementation. The full SVGO plugin uses CSS
/// style computation (computeStyle) to handle inherited display/opacity.
/// This version handles only inline attributes.
pub struct RemoveHiddenElems;

impl Plugin for RemoveHiddenElems {
    fn name(&self) -> &'static str {
        "removeHiddenElems"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        let display_none = params
            .get("displayNone")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let circle_r0 = params
            .get("circleR0")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let ellipse_rx0 = params
            .get("ellipseRX0")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let ellipse_ry0 = params
            .get("ellipseRY0")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let rect_width0 = params
            .get("rectWidth0")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let rect_height0 = params
            .get("rectHeight0")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let pattern_width0 = params
            .get("patternWidth0")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let pattern_height0 = params
            .get("patternHeight0")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let image_width0 = params
            .get("imageWidth0")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let image_height0 = params
            .get("imageHeight0")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let path_empty_d = params
            .get("pathEmptyD")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let polyline_empty_points = params
            .get("polylineEmptyPoints")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let polygon_empty_points = params
            .get("polygonEmptyPoints")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        struct V {
            display_none: bool,
            circle_r0: bool,
            ellipse_rx0: bool,
            ellipse_ry0: bool,
            rect_width0: bool,
            rect_height0: bool,
            pattern_width0: bool,
            pattern_height0: bool,
            image_width0: bool,
            image_height0: bool,
            path_empty_d: bool,
            polyline_empty_points: bool,
            polygon_empty_points: bool,
        }
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                // display="none" - skip markers
                if self.display_none && el.attr("display") == Some("none") && el.name != "marker" {
                    return VisitAction::Remove;
                }

                // visibility="hidden"
                if el.attr("visibility") == Some("hidden") {
                    return VisitAction::Remove;
                }

                // opacity="0" (inline only - no CSS computation)
                if el.attr("opacity") == Some("0") {
                    if el.name == "path" {
                        // paths with opacity 0 are kept for now (could be referenced)
                    } else {
                        return VisitAction::Remove;
                    }
                }

                // Circle with zero radius
                if self.circle_r0
                    && el.name == "circle"
                    && el.children.is_empty()
                    && el.attr("r") == Some("0")
                {
                    return VisitAction::Remove;
                }

                // Ellipse with zero x-axis radius
                if self.ellipse_rx0
                    && el.name == "ellipse"
                    && el.children.is_empty()
                    && el.attr("rx") == Some("0")
                {
                    return VisitAction::Remove;
                }

                // Ellipse with zero y-axis radius
                if self.ellipse_ry0
                    && el.name == "ellipse"
                    && el.children.is_empty()
                    && el.attr("ry") == Some("0")
                {
                    return VisitAction::Remove;
                }

                // Rect with zero width
                if self.rect_width0
                    && el.name == "rect"
                    && el.children.is_empty()
                    && el.attr("width") == Some("0")
                {
                    return VisitAction::Remove;
                }

                // Rect with zero height
                if self.rect_height0
                    && self.rect_width0
                    && el.name == "rect"
                    && el.children.is_empty()
                    && el.attr("height") == Some("0")
                {
                    return VisitAction::Remove;
                }

                // Pattern with zero width
                if self.pattern_width0 && el.name == "pattern" && el.attr("width") == Some("0") {
                    return VisitAction::Remove;
                }

                // Pattern with zero height
                if self.pattern_height0 && el.name == "pattern" && el.attr("height") == Some("0") {
                    return VisitAction::Remove;
                }

                // Image with zero width
                if self.image_width0 && el.name == "image" && el.attr("width") == Some("0") {
                    return VisitAction::Remove;
                }

                // Image with zero height
                if self.image_height0 && el.name == "image" && el.attr("height") == Some("0") {
                    return VisitAction::Remove;
                }

                // Polyline with empty points
                if self.polyline_empty_points
                    && el.name == "polyline"
                    && el.attr("points").is_none()
                {
                    return VisitAction::Remove;
                }

                // Polygon with empty points
                if self.polygon_empty_points && el.name == "polygon" && el.attr("points").is_none()
                {
                    return VisitAction::Remove;
                }

                // Path with empty data
                if self.path_empty_d && el.name == "path" {
                    if el.attr("d").is_none() {
                        return VisitAction::Remove;
                    }
                    // Check if path data is effectively empty
                    let d = el.attr("d").unwrap();
                    if d.trim().is_empty() {
                        return VisitAction::Remove;
                    }
                }

                VisitAction::Continue
            }
        }

        let mut v = V {
            display_none,
            circle_r0,
            ellipse_rx0,
            ellipse_ry0,
            rect_width0,
            rect_height0,
            pattern_width0,
            pattern_height0,
            image_width0,
            image_height0,
            path_empty_d,
            polyline_empty_points,
            polygon_empty_points,
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}
