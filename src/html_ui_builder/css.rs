use lightningcss::properties::Property;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::StyleSheet;
use lightningcss::values::color::CssColor;
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
                            // Simplified - ในการใช้งานจริงต้อง handle ทุก side
                            properties.insert(
                                "padding".to_string(),
                                CssPropertyValue::String(format!("{:?}", padding)),
                            );
                        }
                        Property::Margin(margin) => {
                            properties.insert(
                                "margin".to_string(),
                                CssPropertyValue::String(format!("{:?}", margin)),
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
