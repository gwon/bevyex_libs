use super::css::CssStyleSheet;
use super::utils::{
    compute_element_styles, convert_css_to_bevy_style, extract_background_color, extract_font_size,
    extract_text_color,
};
use bevy::prelude::*;
use std::collections::HashMap;

// Data structures
#[derive(Debug, Clone)]
pub struct UIElement {
    pub tag: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub text: String,
    pub children: Vec<UIElement>,
    pub computed_style: Node,
    pub background_color: BackgroundColor,
    pub text_color: Color,
    pub font_size: f32,
}

impl UIElement {
    pub fn from_html_element(
        element: &scraper::ElementRef,
        stylesheet: &Option<Box<CssStyleSheet>>,
    ) -> Self {
        let value = element.value();

        let tag = value.name().to_string();
        let id = value.attr("id").map(|s| s.to_string());
        let classes: Vec<String> = value
            .attr("class")
            .map(|c| c.split_whitespace().map(|s| s.to_string()).collect())
            .unwrap_or_default();

        let text = element.text().collect::<String>().trim().to_string();

        // Compute styles
        let css_properties = if let Some(stylesheet) = stylesheet {
            compute_element_styles(&tag, &id, &classes, stylesheet)
        } else {
            HashMap::new()
        };

        let computed_style = convert_css_to_bevy_style(&css_properties);
        let background_color = extract_background_color(&css_properties);
        let text_color = extract_text_color(&css_properties);
        let font_size = extract_font_size(&css_properties);
        println!(
            "element: tag:{:?}, id:{:?}, classes:{:?}\n",
            tag, id, classes
        );
        UIElement {
            tag,
            id,
            classes,
            text,
            children: Vec::new(),
            computed_style,
            background_color,
            text_color,
            font_size,
        }
    }

    pub fn from_html_element_with_children(
        element: &scraper::ElementRef,
        stylesheet: &Option<Box<CssStyleSheet>>,
    ) -> Self {
        let value = element.value();

        let tag = value.name().to_string();
        let id = value.attr("id").map(|s| s.to_string());
        let classes: Vec<String> = value
            .attr("class")
            .map(|c| c.split_whitespace().map(|s| s.to_string()).collect())
            .unwrap_or_default();

        // สำหรับ text เฉพาะของ element นี้ (ไม่รวม children)
        let text = element
            .children()
            .filter(|child| child.value().is_text())
            .map(|text_node| text_node.value().as_text().unwrap().as_ref())
            .collect::<Vec<&str>>()
            .join("")
            .trim()
            .to_string();

        // Compute styles
        let css_properties = if let Some(stylesheet) = stylesheet {
            compute_element_styles(&tag, &id, &classes, stylesheet)
        } else {
            HashMap::new()
        };

        let computed_style = convert_css_to_bevy_style(&css_properties);
        let background_color = extract_background_color(&css_properties);
        let text_color = extract_text_color(&css_properties);
        let font_size = extract_font_size(&css_properties);

        // สร้าง children แบบ recursive
        let children: Vec<UIElement> = element
            .children()
            .filter(|child| child.value().is_element())
            .map(|child_element| {
                UIElement::from_html_element_with_children(
                    &scraper::ElementRef::wrap(child_element).unwrap(),
                    stylesheet,
                )
            })
            .collect();

        println!(
            "element: tag:{:?}, id:{:?}, classes:{:?}, children_count:{}\n",
            tag,
            id,
            classes,
            children.len()
        );

        UIElement {
            tag,
            id,
            classes,
            text,
            children,
            computed_style,
            background_color,
            text_color,
            font_size,
        }
    }
}
