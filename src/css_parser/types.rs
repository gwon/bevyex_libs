use bevy::prelude::*;
use bevy::reflect::{Reflect, TypePath};
use std::collections::HashMap;

#[derive(Resource, Default, Reflect, TypePath)]
#[reflect(Resource)]
pub struct CssRules {
    pub rules: HashMap<String, StyleProperties>,
}

#[derive(Debug, Clone, Default, Reflect, TypePath)]
pub struct StyleProperties {
    pub display: Option<Display>,
    pub position_type: Option<PositionType>,
    pub direction: Option<Direction>,
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<FlexWrap>,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,
    pub align_content: Option<AlignContent>,
    pub justify_content: Option<JustifyContent>,
    pub position: UiRect<Val>,
    pub margin: UiRect<Val>,
    pub padding: UiRect<Val>,
    pub border: UiRect<Val>,
    pub width: Option<Val>,
    pub height: Option<Val>,
    pub min_width: Option<Val>,
    pub max_width: Option<Val>,
    pub min_height: Option<Val>,
    pub max_height: Option<Val>,
    pub aspect_ratio: Option<f32>,
    pub overflow: Option<Overflow>,
    pub background_color: Option<BackgroundColor>,
    pub custom_property: Option<String>,
}

impl From<StyleProperties> for Style {
    fn from(props: StyleProperties) -> Self {
        Style {
            display: props.display.unwrap_or(Default::default()),
            position_type: props.position_type.unwrap_or(Default::default()),
            direction: props.direction.unwrap_or(Default::default()),
            flex_direction: props.flex_direction.unwrap_or(Default::default()),
            flex_wrap: props.flex_wrap.unwrap_or(Default::default()),
            align_items: props.align_items.unwrap_or(Default::default()),
            align_self: props.align_self.unwrap_or(Default::default()),
            align_content: props.align_content.unwrap_or(Default::default()),
            justify_content: props.justify_content.unwrap_or(Default::default()),
            position: props.position,
            margin: props.margin,
            padding: props.padding,
            border: props.border,
            width: props.width.unwrap_or(Default::default()),
            height: props.height.unwrap_or(Default::default()),
            min_width: props.min_width.unwrap_or(Default::default()),
            max_width: props.max_width.unwrap_or(Default::default()),
            min_height: props.min_height.unwrap_or(Default::default()),
            max_height: props.max_height.unwrap_or(Default::default()),
            aspect_ratio: props.aspect_ratio,
            overflow: props.overflow.unwrap_or(Default::default()),
            ..Default::default()
        }
    }
}

#[derive(Component, Reflect, TypePath)]
pub struct CssSelector(pub String);
