use super::css::CssStyleSheet;
use super::element::UIElement;
use bevy::prelude::*;
use lightningcss::{
    rules::CssRule,
    stylesheet::{ParserOptions, StyleSheet},
};

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

        // Select เฉพาะ elements ใน body
        let body_selector = Selector::parse("body *").unwrap();

        for element in document.select(&body_selector) {
            let ui_element = UIElement::from_html_element(&element, stylesheet);
            elements.push(ui_element);
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
                let entity =
                    self.spawn_element_with_children(commands, asset_server, element, elements);
                commands.entity(root).add_child(entity);
                break; // ใช้แค่ container หลัก
            }
        }
    }

    fn spawn_element_with_children(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        element: &UIElement,
        all_elements: &[UIElement],
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
                parent.spawn((
                    Text::new(element.text.clone()),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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

        // หา children elements และสร้างพวกมัน
        for child_element in all_elements {
            if self.is_child_of(child_element, element) {
                let child_entity = self.spawn_element(commands, asset_server, child_element);
                commands.entity(entity_id).add_child(child_entity);
            }
        }

        entity_id
    }

    fn is_child_of(&self, potential_child: &UIElement, parent: &UIElement) -> bool {
        // สำหรับตัวอย่างนี้ เราจะใช้ logic ง่ายๆ:
        // h1 อยู่ใน main-content div
        // card divs อยู่ใน main-content div
        // p และ button อยู่ใน card divs

        if parent.id == Some("main-content".to_string()) {
            return potential_child.tag == "h1"
                || (potential_child.tag == "div"
                    && potential_child.classes.contains(&"card".to_string()));
        }

        if parent.tag == "div" && parent.classes.contains(&"card".to_string()) {
            return potential_child.tag == "p" || potential_child.tag == "button";
        }

        false
    }

    fn spawn_element(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        element: &UIElement,
    ) -> Entity {
        let mut entity_commands =
            commands.spawn((element.computed_style.clone(), element.background_color));

        // เพิ่ม text ถ้าเป็น text elements
        if !element.text.is_empty()
            && matches!(
                element.tag.as_str(),
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "span" | "button"
            )
        {
            entity_commands.with_children(|parent| {
                parent.spawn((
                    Text::new(element.text.clone()),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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

        entity_commands.id()
    }
}
