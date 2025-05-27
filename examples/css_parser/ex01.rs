use crate::css_parser::{CssPlugin, CssRules, CssSelector, parse_stylesheet};
use bevy::prelude::*; // สมมติว่าอยู่ใน library เดียวกัน
// หากเป็น crate แยก: use bevy_css_parser::{CssPlugin, CssSelector, CssRules, parse_stylesheet};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CssPlugin) // เพิ่มปลั๊กอิน CSS ของเรา
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // ตัวอย่าง CSS string
    let css_string = "
        .container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            width: 100%;
            height: 100%;
            background-color: #121212;
        }

        .text {
            font-size: 20px;
            color: white;
            margin-bottom: 10px;
        }
    ";

    // สร้าง resource สำหรับเก็บกฎ CSS
    let mut css_rules = CssRules::default();

    // เรียกใช้ parse_stylesheet เพื่อประมวลผล CSS string
    parse_stylesheet(css_string, &mut css_rules.rules);

    // เพิ่ม resource CssRules ลงใน Bevy app
    commands.insert_resource(css_rules);

    // สร้าง UI และกำหนด CssSelector
    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..Default::default()
            },
            CssSelector(".container".to_string()), // กำหนด selector
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Hello Bevy with Custom CSS!",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"), // อย่าลืมใส่ font
                    },
                ),
                CssSelector(".text".to_string()), // กำหนด selector
            ));
        })
        .id();

    println!(
        "Parsed CSS Rules in setup: {:?}",
        commands.get_resource::<CssRules>()
    );
}
