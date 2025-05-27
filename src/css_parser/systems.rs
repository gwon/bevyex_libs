use super::parser::parse_stylesheet;
use super::types::{CssRules, CssSelector};
use bevy::prelude::*;

pub fn load_css_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut css_rules: ResMut<CssRules>,
) {
    let css_handle = asset_server.load("styles.css");
    let css_string = asset_server.get_handle::<String>(css_handle);

    if let Some(css_string) = css_string {
        parse_stylesheet(&css_string, &mut css_rules.rules);
        println!("Parsed CSS Rules: {:?}", css_rules.rules);
    } else {
        println!("CSS Not Loaded Yet");
    }
}

pub fn apply_css_system(
    mut commands: Commands,
    css_rules: Res<CssRules>,
    query: Query<(Entity, &CssSelector, &mut Style)>,
) {
    for (entity, selector, mut style) in query.iter() {
        if let Some(props) = css_rules.rules.get(&selector.0) {
            *style = Style::from(props.clone());
        }
    }
}

pub struct CssPlugin;

impl Plugin for CssPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CssRules>()
            .add_systems(Startup, load_css_system)
            .add_systems(Update, apply_css_system);
    }
}
