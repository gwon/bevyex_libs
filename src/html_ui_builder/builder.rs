use super::css::CssStyleSheet;
use super::element::UIElement;
use bevy::prelude::*;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};

use scraper::{Html, Selector};

// Main UI Builder struct
pub struct HtmlCssUIBuilder {
    stylesheet: Option<Box<CssStyleSheet>>,
    owned_css: Option<String>,
}

impl HtmlCssUIBuilder {
    pub fn new() -> Self {
        Self {
            stylesheet: None,
            owned_css: None,
        }
    }

    pub fn parse_and_build(
        &mut self,
        html_content: &str,
    ) -> Result<Vec<UIElement>, Box<dyn std::error::Error>> {
        let document = Html::parse_document(html_content);

        // Extract และ parse CSS
        let css_content = self.extract_css(&document);
        let stylesheet = if !css_content.is_empty() {
            Some(self.parse_css(&css_content)?)
        } else {
            None
        };

        // Parse HTML elements
        let elements = self.parse_html_elements(&document, &stylesheet);

        Ok(elements)
    }

    fn extract_css(&self, document: &Html) -> String {
        let style_selector = Selector::parse("style").unwrap();
        let mut css_content = String::new();

        for style_element in document.select(&style_selector) {
            css_content.push_str(&style_element.inner_html());
            css_content.push('\n');
        }

        css_content
    }

    fn parse_css(
        &mut self,
        css_content: &str,
    ) -> Result<Box<CssStyleSheet>, Box<dyn std::error::Error>> {
        let options = ParserOptions::default();
        let owned_css = css_content.to_string();
        let static_css: &'static str = Box::leak(owned_css.into_boxed_str());
        let stylesheet = StyleSheet::parse(static_css, options)?;
        Ok(Box::new(CssStyleSheet::from_lightningcss(stylesheet)))
    }

    fn parse_html_elements(
        &self,
        document: &Html,
        stylesheet: &Option<Box<CssStyleSheet>>,
    ) -> Vec<UIElement> {
        let mut elements = Vec::new();

        // Select เฉพาะ body element เพื่อสร้าง tree
        let body_selector = Selector::parse("body").unwrap();

        if let Some(body_element) = document.select(&body_selector).next() {
            // สร้าง children ของ body แบบ recursive
            for child in body_element.children() {
                if child.value().is_element() {
                    let child_ref = scraper::ElementRef::wrap(child).unwrap();
                    let ui_element =
                        UIElement::from_html_element_with_children(&child_ref, stylesheet);
                    elements.push(ui_element);
                }
            }
        }

        elements
    }

    pub fn spawn_bevy_ui(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        elements: &[UIElement],
    ) {
        // สร้าง root container
        let root = commands
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            })
            .id();

        // สร้าง UI hierarchy แบบ recursive
        for element in elements {
            if element.tag == "div" && element.classes.contains(&"container".to_string()) {
                let entity = self.spawn_element_recursive(commands, asset_server, element);
                commands.entity(root).add_child(entity);
                break; // ใช้แค่ container หลัก
            }
        }
    }

    fn spawn_element_recursive(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        element: &UIElement,
    ) -> Entity {
        let mut entity_commands =
            commands.spawn((element.computed_style.clone(), element.background_color));

        // เพิ่ม text ถ้าเป็น text elements เช่น h1, p, button
        if !element.text.is_empty()
            && matches!(
                element.tag.as_str(),
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "span" | "button"
            )
        {
            entity_commands.with_children(|parent| {
                // พยายามโหลด font file ก่อน
                let font_path = "fonts/FiraSans-Bold.ttf";
                let font_handle = asset_server.load(font_path);

                // ตรวจสอบว่า font โหลดได้หรือไม่ ถ้าไม่ได้ใช้ default
                let final_font_handle =
                    if std::path::Path::new(&format!("assets/{}", font_path)).exists() {
                        font_handle
                    } else {
                        Handle::default()
                    };

                parent.spawn((
                    Text::new(element.text.clone()),
                    TextFont {
                        font: final_font_handle,
                        font_size: element.font_size,
                        ..default()
                    },
                    TextColor(element.text_color),
                ));
            });
        }

        // เพิ่ม interaction สำหรับ button
        if element.classes.contains(&"button".to_string()) || element.tag == "button" {
            entity_commands.insert(Interaction::default());
        }

        let entity_id = entity_commands.id();

        // สร้าง children แบบ recursive
        for child_element in &element.children {
            let child_entity = self.spawn_element_recursive(commands, asset_server, child_element);
            commands.entity(entity_id).add_child(child_entity);
        }

        entity_id
    }
}
