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
    let mut node = Node::default();

    if let Some(CssPropertyValue::Size(value)) = properties.get("width") {
        node.width = Val::Px(*value);
    }

    if let Some(CssPropertyValue::Size(value)) = properties.get("height") {
        node.height = Val::Px(*value);
    }

    // Handle padding
    if let Some(CssPropertyValue::Rect {
        top,
        right,
        bottom,
        left,
    }) = properties.get("padding")
    {
        println!(
            "xxx padding: {:?}, {:?}, {:?}, {:?}",
            *top, *right, *bottom, *left
        );
        node.padding = UiRect::new(*left, *right, *top, *bottom);
    }

    // Handle margin
    if let Some(CssPropertyValue::Rect {
        top,
        right,
        bottom,
        left,
    }) = properties.get("margin")
    {
        println!(
            "xxx margin: {:?}, {:?}, {:?}, {:?}",
            *top, *right, *bottom, *left
        );
        node.margin = UiRect::new(*left, *right, *top, *bottom);
    }

    // Default layout for containers
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;

    node
}

pub fn extract_background_color(properties: &HashMap<String, CssPropertyValue>) -> BackgroundColor {
    if let Some(CssPropertyValue::Color(color)) = properties.get("background-color") {
        BackgroundColor(css_color_to_bevy_color(color))
    } else {
        BackgroundColor::default()
    }
}

pub fn extract_border_radius(properties: &HashMap<String, CssPropertyValue>) -> BorderRadius {
    if let Some(CssPropertyValue::Corner {
        top_left,
        top_right,
        bottom_right,
        bottom_left,
    }) = properties.get("border-radius")
    {
        BorderRadius::new(*top_left, *top_right, *bottom_right, *bottom_left)
    } else {
        BorderRadius::default()
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
    let size = properties.get("font-size");
    match size {
        Some(CssPropertyValue::Size(size)) => *size,
        _ => 16.0,
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
