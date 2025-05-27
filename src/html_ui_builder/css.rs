use lightningcss::properties::Property;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::StyleSheet;
use lightningcss::values::color::CssColor;
use lightningcss::values::length::LengthPercentage;
use lightningcss::values::length::LengthValue;
use lightningcss::values::percentage::DimensionPercentage;
use lightningcss::values::size::Size2D;
use std::collections::HashMap;
use std::default::Default;

#[derive(Debug, Default)]
pub struct CssStyleSheet {
    pub rules: Vec<CssRule_>,
}

#[derive(Debug, Clone)]
pub struct CssRule_ {
    pub selector: String,
    pub properties: HashMap<String, CssPropertyValue>,
}

#[derive(Debug, Clone)]
pub enum CssPropertyValue {
    Color(CssColor),
    Length(f32, String), // value, unit
    String(String),
    Number(f32),
    Padding {
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
    },
    Margin {
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
    },
    BorderRadius {
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    },
}

impl CssStyleSheet {
    pub fn from_lightningcss(stylesheet: StyleSheet) -> Self {
        let mut rules = Vec::new();

        for rule in &stylesheet.rules.0 {
            if let CssRule::Style(style_rule) = rule {
                let selector_str = style_rule.selectors.to_string();
                let mut properties = HashMap::new();

                for declaration in &style_rule.declarations.declarations {
                    match declaration {
                        Property::BackgroundColor(color) => {
                            properties.insert(
                                "background-color".to_string(),
                                CssPropertyValue::Color(color.clone()),
                            );
                        }
                        Property::Color(color) => {
                            properties.insert(
                                "color".to_string(),
                                CssPropertyValue::Color(color.clone()),
                            );
                        }
                        Property::FontSize(size) => {
                            if let Some((value, unit)) = extract_font_size_value(size) {
                                properties.insert(
                                    "font-size".to_string(),
                                    CssPropertyValue::Length(value, unit),
                                );
                            }
                        }
                        Property::Width(width) => {
                            if let Some((value, unit)) = extract_size_value(width) {
                                properties.insert(
                                    "width".to_string(),
                                    CssPropertyValue::Length(value, unit),
                                );
                            }
                        }
                        Property::Height(height) => {
                            if let Some((value, unit)) = extract_size_value(height) {
                                properties.insert(
                                    "height".to_string(),
                                    CssPropertyValue::Length(value, unit),
                                );
                            }
                        }
                        Property::Padding(padding) => {
                            let (top, right, bottom, left) =
                                extract_padding_values(&Property::Padding(padding.clone()));
                            properties.insert(
                                "padding".to_string(),
                                CssPropertyValue::Padding {
                                    top,
                                    right,
                                    bottom,
                                    left,
                                },
                            );
                        }
                        Property::Margin(margin) => {
                            let (top, right, bottom, left) =
                                extract_margin_values(&Property::Margin(margin.clone()));
                            properties.insert(
                                "margin".to_string(),
                                CssPropertyValue::Margin {
                                    top,
                                    right,
                                    bottom,
                                    left,
                                },
                            );
                        }
                        Property::BorderRadius(border_radius, _) => {
                            let (top_left, top_right, bottom_right, bottom_left) =
                                extract_border_radius_values(&border_radius);
                            properties.insert(
                                "border-radius".to_string(),
                                CssPropertyValue::BorderRadius {
                                    top_left,
                                    top_right,
                                    bottom_right,
                                    bottom_left,
                                },
                            );
                        }
                        _ => {} // Handle other properties as needed
                    }
                }

                rules.push(CssRule_ {
                    selector: selector_str,
                    properties,
                });
            }
        }

        CssStyleSheet { rules }
    }
}

// Helper functions
fn extract_font_size_value(
    size: &lightningcss::properties::font::FontSize,
) -> Option<(f32, String)> {
    // ใช้ debug format เป็นทางเลือกชั่วคราว
    let size_str = format!("{:?}", size);
    if size_str.contains("Px(") {
        // Extract px value from debug string
        if let Some(start) = size_str.find("Px(") {
            if let Some(end) = size_str[start + 3..].find(')') {
                if let Ok(value) = size_str[start + 3..start + 3 + end].parse::<f32>() {
                    return Some((value, "px".to_string()));
                }
            }
        }
    }
    Some((16.0, "px".to_string())) // default fallback
}

fn extract_size_value(size: &lightningcss::properties::size::Size) -> Option<(f32, String)> {
    // ใช้ debug format เป็นทางเลือกชั่วคราว
    let size_str = format!("{:?}", size);
    if size_str.contains("Px(") {
        if let Some(start) = size_str.find("Px(") {
            if let Some(end) = size_str[start + 3..].find(')') {
                if let Ok(value) = size_str[start + 3..start + 3 + end].parse::<f32>() {
                    return Some((value, "px".to_string()));
                }
            }
        }
    }
    if size_str.contains("Percentage(") {
        if let Some(start) = size_str.find("Percentage(") {
            if let Some(end) = size_str[start + 11..].find(')') {
                if let Ok(value) = size_str[start + 11..start + 11 + end].parse::<f32>() {
                    return Some((value, "%".to_string()));
                }
            }
        }
    }
    Some((100.0, "px".to_string())) // default fallback
}

fn extract_length_value(
    size: &lightningcss::values::length::LengthPercentageOrAuto,
) -> Option<(f32, String)> {
    match size {
        lightningcss::values::length::LengthPercentageOrAuto::LengthPercentage(lp) => match lp {
            lightningcss::values::length::LengthPercentage::Dimension(l) => {
                Some((l.to_px().unwrap_or(0.0), "px".to_string()))
            }
            lightningcss::values::length::LengthPercentage::Percentage(p) => {
                Some((p.0 * 100.0, "%".to_string()))
            }
            lightningcss::values::length::LengthPercentage::Calc(_) => {
                Some((0.0, "px".to_string()))
            }
        },
        _ => Some((0.0, "px".to_string())),
    }
}

fn extract_value(dim_pct: &DimensionPercentage<lightningcss::values::length::LengthValue>) -> f32 {
    match dim_pct {
        DimensionPercentage::Dimension(len) => len.to_px().unwrap_or(0.0),
        DimensionPercentage::Percentage(pct) => pct.0,
        DimensionPercentage::Calc(_) => 0.0,
    }
}

fn extract_border_radius_values(
    border_radius: &lightningcss::properties::border_radius::BorderRadius,
) -> (f32, f32, f32, f32) {
    let top_left = extract_value(&border_radius.top_left.0);
    let top_right = extract_value(&border_radius.top_right.0);
    let bottom_right = extract_value(&border_radius.bottom_right.0);
    let bottom_left = extract_value(&border_radius.bottom_left.0);
    (top_left, top_right, bottom_right, bottom_left)
}

fn extract_padding_values(padding: &Property<'_>) -> (f32, f32, f32, f32) {
    match padding {
        Property::Padding(p) => {
            let top = extract_length_value(&p.top).map_or(0.0, |(v, _)| v);
            let right = extract_length_value(&p.right).map_or(0.0, |(v, _)| v);
            let bottom = extract_length_value(&p.bottom).map_or(0.0, |(v, _)| v);
            let left = extract_length_value(&p.left).map_or(0.0, |(v, _)| v);
            (top, right, bottom, left)
        }
        _ => (0.0, 0.0, 0.0, 0.0),
    }
}

fn extract_margin_values(margin: &Property<'_>) -> (f32, f32, f32, f32) {
    match margin {
        Property::Margin(m) => {
            let top = extract_length_value(&m.top).map_or(0.0, |(v, _)| v);
            let right = extract_length_value(&m.right).map_or(0.0, |(v, _)| v);
            let bottom = extract_length_value(&m.bottom).map_or(0.0, |(v, _)| v);
            let left = extract_length_value(&m.left).map_or(0.0, |(v, _)| v);
            (top, right, bottom, left)
        }
        _ => (0.0, 0.0, 0.0, 0.0),
    }
}
