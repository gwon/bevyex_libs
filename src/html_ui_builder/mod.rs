// Main UI Builder struct
pub struct HtmlCssUIBuilder {
    stylesheet: Option<CssStyleSheet>,
}

impl HtmlCssUIBuilder {
    pub fn new() -> Self {
        Self { stylesheet: None }
    }

    pub fn parse_and_build(
        &self,
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

    fn parse_css(&self, css_content: &str) -> Result<CssStyleSheet, Box<dyn std::error::Error>> {
        let options = ParserOptions::default();
        let stylesheet = StyleSheet::parse(css_content, options)?;
        Ok(CssStyleSheet::from_lightningcss(&stylesheet))
    }

    fn parse_html_elements(
        &self,
        document: &Html,
        stylesheet: &Option<CssStyleSheet>,
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
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .id();

        // สร้าง UI hierarchy
        for element in elements {
            if element.tag == "div" && element.classes.contains(&"container".to_string()) {
                let entity = self.spawn_element(commands, asset_server, element);
                commands.entity(root).add_child(entity);
                break; // ใช้แค่ container หลัก
            }
        }
    }

    fn spawn_element(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        element: &UIElement,
    ) -> Entity {
        let mut entity_commands = commands.spawn(NodeBundle {
            style: element.computed_style.clone(),
            background_color: element.background_color,
            ..default()
        });

        // เพิ่ม text ถ้ามี
        if !element.text.is_empty() && !element.tag.starts_with("div") {
            entity_commands.with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    element.text.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: element.font_size,
                        color: element.text_color,
                    },
                ));
            });
        }

        // เพิ่ม interaction สำหรับ button
        if element.classes.contains(&"button".to_string()) {
            entity_commands.insert(Interaction::default());
        }

        entity_commands.id()
    }
}

// Data structures
#[derive(Debug, Clone)]
pub struct UIElement {
    pub tag: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub text: String,
    pub computed_style: Style,
    pub background_color: BackgroundColor,
    pub text_color: Color,
    pub font_size: f32,
}

impl UIElement {
    pub fn from_html_element(
        element: &scraper::ElementRef,
        stylesheet: &Option<CssStyleSheet>,
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

        UIElement {
            tag,
            id,
            classes,
            text,
            computed_style,
            background_color,
            text_color,
            font_size,
        }
    }
}

#[derive(Debug)]
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
    pub fn from_lightningcss(stylesheet: &StyleSheet) -> Self {
        let mut rules = Vec::new();

        for rule in &stylesheet.rules.0 {
            if let CssRule::Style(style_rule) = rule {
                for selector in &style_rule.selectors {
                    let selector_str = format_css_selector(selector);
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
                                if let Some((value, unit)) = extract_length_value(size) {
                                    properties.insert(
                                        "font-size".to_string(),
                                        CssPropertyValue::Length(value, unit),
                                    );
                                }
                            }
                            Property::Width(width) => {
                                if let Some((value, unit)) = extract_length_percentage_value(width)
                                {
                                    properties.insert(
                                        "width".to_string(),
                                        CssPropertyValue::Length(value, unit),
                                    );
                                }
                            }
                            Property::Height(height) => {
                                if let Some((value, unit)) = extract_length_percentage_value(height)
                                {
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
        }

        CssStyleSheet { rules }
    }
}

// Helper functions
fn format_css_selector(selector: &CssSelector) -> String {
    let mut result = String::new();
    for component in &selector.components {
        match component {
            Component::Class(class) => {
                result.push('.');
                result.push_str(class);
            }
            Component::Id(id) => {
                result.push('#');
                result.push_str(id);
            }
            Component::LocalName(name) => {
                result.push_str(&name.name);
            }
            _ => {}
        }
    }
    result
}

fn extract_length_value(length: &Length) -> Option<(f32, String)> {
    match &length.value {
        LengthValue::Px(px) => Some((*px, "px".to_string())),
        LengthValue::Em(em) => Some((*em, "em".to_string())),
        LengthValue::Rem(rem) => Some((*rem, "rem".to_string())),
        _ => None,
    }
}

fn extract_length_percentage_value(lp: &LengthPercentage) -> Option<(f32, String)> {
    match lp {
        LengthPercentage::Length(length) => extract_length_value(length),
        LengthPercentage::Percentage(percentage) => Some((*percentage, "%".to_string())),
        _ => None,
    }
}

fn compute_element_styles(
    tag: &str,
    id: &Option<String>,
    classes: &[String],
    stylesheet: &CssStyleSheet,
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

fn convert_css_to_bevy_style(properties: &HashMap<String, CssPropertyValue>) -> Style {
    let mut style = Style::default();

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

fn extract_background_color(properties: &HashMap<String, CssPropertyValue>) -> BackgroundColor {
    if let Some(CssPropertyValue::Color(color)) = properties.get("background-color") {
        BackgroundColor(css_color_to_bevy_color(color))
    } else {
        BackgroundColor::default()
    }
}

fn extract_text_color(properties: &HashMap<String, CssPropertyValue>) -> Color {
    if let Some(CssPropertyValue::Color(color)) = properties.get("color") {
        css_color_to_bevy_color(color)
    } else {
        Color::BLACK
    }
}

fn extract_font_size(properties: &HashMap<String, CssPropertyValue>) -> f32 {
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

fn css_color_to_bevy_color(css_color: &CssColor) -> Color {
    match css_color {
        CssColor::RGBA(rgba) => Color::rgba(rgba.red, rgba.green, rgba.blue, rgba.alpha),
        _ => Color::BLACK,
    }
}
