use super::css::{CssPropertyValue, CssStyleSheet};
use bevy::prelude::*;
use lightningcss::values::color::CssColor;
use std::collections::HashMap;

pub fn compute_element_styles(
    tag: &str,
    id: &Option<String>,
    classes: &[String],
    stylesheet: &Box<CssStyleSheet>,
) -> HashMap<String, CssPropertyValue> {
    let mut computed = HashMap::new();

    // Apply tag styles
    for rule in &stylesheet.rules {
        if rule.selector == tag {
            for (prop, value) in &rule.properties {
                computed.insert(prop.clone(), value.clone());
            }
        }
    }

    // Apply class styles
    for class in classes {
        let class_selector = format!(".{}", class);
        for rule in &stylesheet.rules {
            if rule.selector == class_selector {
                for (prop, value) in &rule.properties {
                    computed.insert(prop.clone(), value.clone());
                }
            }
        }
    }

    // Apply ID styles (highest specificity)
    if let Some(id) = id {
        let id_selector = format!("#{}", id);
        for rule in &stylesheet.rules {
            if rule.selector == id_selector {
                for (prop, value) in &rule.properties {
                    computed.insert(prop.clone(), value.clone());
                }
            }
        }
    }

    computed
}

pub fn convert_css_to_bevy_style(properties: &HashMap<String, CssPropertyValue>) -> Node {
    let mut style = Node::default();

    if let Some(CssPropertyValue::Length(value, unit)) = properties.get("width") {
        style.width = match unit.as_str() {
            "px" => Val::Px(*value),
            "%" => Val::Percent(*value),
            _ => Val::Auto,
        };
    }

    if let Some(CssPropertyValue::Length(value, unit)) = properties.get("height") {
        style.height = match unit.as_str() {
            "px" => Val::Px(*value),
            "%" => Val::Percent(*value),
            _ => Val::Auto,
        };
    }

    // Default layout for containers
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;

    style
}

pub fn extract_background_color(properties: &HashMap<String, CssPropertyValue>) -> BackgroundColor {
    if let Some(CssPropertyValue::Color(color)) = properties.get("background-color") {
        BackgroundColor(css_color_to_bevy_color(color))
    } else {
        BackgroundColor::default()
    }
}

pub fn extract_text_color(properties: &HashMap<String, CssPropertyValue>) -> Color {
    if let Some(CssPropertyValue::Color(color)) = properties.get("color") {
        css_color_to_bevy_color(color)
    } else {
        Color::BLACK
    }
}

pub fn extract_font_size(properties: &HashMap<String, CssPropertyValue>) -> f32 {
    if let Some(CssPropertyValue::Length(size, unit)) = properties.get("font-size") {
        match unit.as_str() {
            "px" => *size,
            "em" => *size * 16.0,
            "rem" => *size * 16.0,
            _ => 16.0,
        }
    } else {
        16.0
    }
}

pub fn css_color_to_bevy_color(css_color: &CssColor) -> Color {
    match css_color {
        CssColor::RGBA(rgba) => Color::srgba(
            rgba.red as f32 / 255.0,
            rgba.green as f32 / 255.0,
            rgba.blue as f32 / 255.0,
            rgba.alpha as f32 / 255.0,
        ),
        _ => Color::BLACK,
    }
}
