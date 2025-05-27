use bevy::prelude::*;
use bevy::reflect::{Reflect, TypePath};
use cssparser::{BasicParseErrorHandler, ParseError, ParseErrorKind, Parser, Token};
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CssPlugin) // เพิ่มปลั๊กอิน CSS ของเรา
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut css_rules: ResMut<CssRules>) {
    //spawn camera
    commands.spawn(Camera2dBundle::default());

    // โหลด CSS จาก string โดยตรง
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
        .title {
            font-size: 30px;
            color: white;
            margin-bottom: 20px;
        }
        .button {
            padding: 10px 20px;
            background-color: #4CAF50;
            color: white;
            border: none;
            font-size: 20px;
            cursor: pointer;
        }
        .button:hover {
            background-color: #367c39;
        }
        #special_button {
            background-color: blue;
        }
    ";

    // ใช้ parse_stylesheet เพื่อแยกวิเคราะห์ CSS string และเก็บไว้ใน resource CssRules
    parse_stylesheet(css_string, &mut css_rules.rules);

    // พิมพ์กฎ CSS ที่แยกวิเคราะห์เพื่อตรวจสอบ
    println!("Parsed CSS Rules: {:?}", css_rules.rules);

    // สร้าง UI node และกำหนด selector CSS
    let container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            CssSelector(".container".to_string()), // กำหนด selector CSS
            BackgroundColor(Color::rgb(0.08, 0.08, 0.08)),
        ))
        .id();

    let title = commands
        .spawn((
            TextBundle::from_section(
                "My Bevy UI with CSS",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                },
            ),
            CssSelector(".title".to_string()), // กำหนด selector CSS
            Style {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..Default::default()
            },
        ))
        .id();

    let button = commands
        .spawn((
            TextBundle::from_section(
                "Click Me",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                },
            ),
            CssSelector(".button".to_string()), // กำหนด selector CSS
            Style {
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
        ))
        .id();

    let special_button = commands
        .spawn((
            TextBundle::from_section(
                "Special Click",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                },
            ),
            CssSelector("#special_button".to_string()), // กำหนด selector CSS
            Style {
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
        ))
        .id();

    commands
        .entity(container)
        .push_children(&[title, button, special_button]);
}
