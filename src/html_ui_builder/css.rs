use bevy::ui::Val;
use lightningcss::properties::Property;
use lightningcss::properties::font::{AbsoluteFontSize, FontSize, RelativeFontSize};
use lightningcss::properties::size::Size;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::StyleSheet;
use lightningcss::values::color::CssColor;
use lightningcss::values::percentage::DimensionPercentage;
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
    Size(f32),
    String(String),
    Rect {
        top: Val,
        right: Val,
        bottom: Val,
        left: Val,
    },
    Corner {
        top_left: Val,
        top_right: Val,
        bottom_right: Val,
        bottom_left: Val,
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
                            let font_size = extract_font_size_value(size);
                            properties
                                .insert("font-size".to_string(), CssPropertyValue::Size(font_size));
                        }
                        Property::Width(width) => {
                            let value = extract_size_value(width);
                            properties.insert("width".to_string(), CssPropertyValue::Size(value));
                        }
                        Property::Height(height) => {
                            let value = extract_size_value(height);
                            properties.insert("height".to_string(), CssPropertyValue::Size(value));
                        }
                        Property::Padding(padding) => {
                            let (top, right, bottom, left) =
                                extract_rect_values(&Property::Padding(padding.clone()));
                            properties.insert(
                                "padding".to_string(),
                                CssPropertyValue::Rect {
                                    top,
                                    right,
                                    bottom,
                                    left,
                                },
                            );
                        }
                        Property::Margin(margin) => {
                            let (top, right, bottom, left) =
                                extract_rect_values(&Property::Margin(margin.clone()));
                            properties.insert(
                                "margin".to_string(),
                                CssPropertyValue::Rect {
                                    top,
                                    right,
                                    bottom,
                                    left,
                                },
                            );
                        }
                        Property::BorderRadius(border_radius, _) => {
                            let (top_left, top_right, bottom_right, bottom_left) =
                                extract_corner_values(&border_radius);
                            properties.insert(
                                "border-radius".to_string(),
                                CssPropertyValue::Corner {
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
fn extract_font_size_value(size: &lightningcss::properties::font::FontSize) -> f32 {
    match size {
        FontSize::Length(length) => match length {
            DimensionPercentage::Dimension(len) => len.to_px().unwrap_or(0.0),
            DimensionPercentage::Percentage(pct) => pct.0,
            DimensionPercentage::Calc(_) => 0.0,
        },
        FontSize::Absolute(abs) => match abs {
            AbsoluteFontSize::XXSmall => 10.0,
            AbsoluteFontSize::XSmall => 13.0,
            AbsoluteFontSize::Small => 16.0,
            AbsoluteFontSize::Medium => 20.0,
            AbsoluteFontSize::Large => 24.0,
            AbsoluteFontSize::XLarge => 28.0,
            AbsoluteFontSize::XXLarge => 28.0,
            AbsoluteFontSize::XXXLarge => 32.0,
        },
        FontSize::Relative(rel) => match rel {
            RelativeFontSize::Smaller => 12.0,
            RelativeFontSize::Larger => 16.0,
        },
        _ => 16.0,
    }
}

fn extract_size_value(size: &lightningcss::properties::size::Size) -> f32 {
    match size {
        Size::LengthPercentage(dim_pct) => {
            match dim_pct {
                DimensionPercentage::Dimension(len) => {
                    // แปลงเป็น px
                    len.to_px().unwrap_or(0.0)
                }
                DimensionPercentage::Percentage(pct) => pct.0 * 100.0,
                DimensionPercentage::Calc(_) => {
                    // handle calc() expressions
                    10.0
                }
            }
        }
        Size::Auto => 10.0,
        Size::MaxContent(_) => 10.0,
        Size::MinContent(_) => 10.0,
        Size::FitContent(_) => 10.0,
        Size::Stretch(_) => 10.0,
        _ => 10.0,
    }
}

fn extract_length_value(size: &lightningcss::values::length::LengthPercentageOrAuto) -> Val {
    match size {
        lightningcss::values::length::LengthPercentageOrAuto::LengthPercentage(lp) => match lp {
            lightningcss::values::length::LengthPercentage::Dimension(l) => {
                Val::Px(l.to_px().unwrap_or(0.0))
            }
            lightningcss::values::length::LengthPercentage::Percentage(p) => {
                Val::Percent(p.0 * 100.0)
            }
            lightningcss::values::length::LengthPercentage::Calc(_) => Val::Px(0.0),
        },
        _ => Val::Px(0.0),
    }
}

fn extract_value(dim_pct: &DimensionPercentage<lightningcss::values::length::LengthValue>) -> Val {
    match dim_pct {
        DimensionPercentage::Dimension(len) => Val::Px(len.to_px().unwrap_or(0.0)),
        DimensionPercentage::Percentage(pct) => Val::Percent(pct.0),
        DimensionPercentage::Calc(_) => Val::Px(0.0),
    }
}

fn extract_corner_values(
    border_radius: &lightningcss::properties::border_radius::BorderRadius,
) -> (Val, Val, Val, Val) {
    let top_left = extract_value(&border_radius.top_left.0);
    let top_right = extract_value(&border_radius.top_right.0);
    let bottom_right = extract_value(&border_radius.bottom_right.0);
    let bottom_left = extract_value(&border_radius.bottom_left.0);
    (top_left, top_right, bottom_right, bottom_left)
}

fn extract_rect_values(padding: &Property<'_>) -> (Val, Val, Val, Val) {
    match padding {
        Property::Padding(p) => {
            let top = extract_length_value(&p.top);
            let right = extract_length_value(&p.right);
            let bottom = extract_length_value(&p.bottom);
            let left = extract_length_value(&p.left);
            (top, right, bottom, left)
        }
        _ => (Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
    }
}
