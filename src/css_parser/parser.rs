use super::types::{CssRules, StyleProperties};
use bevy::prelude::*;
use cssparser::{BasicParseErrorHandler, ParseError, ParseErrorKind, Parser, Token};
use std::collections::HashMap;

pub fn parse_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Val, ParseError<'i, BasicParseErrorHandler<'i>>> {
    let location = input.current_source_location();
    match input.next() {
        Ok(Token::Dimension { value, unit, .. }) => match unit.to_lowercase().as_str() {
            "px" => Ok(Val::Px(value)),
            "%" => Ok(Val::Percent(value)),
            _ => Err(ParseError::with_location(
                ParseErrorKind::InvalidToken,
                location,
            )),
        },
        Ok(Token::Number { value, .. }) => Ok(Val::Px(value)),
        Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
            "auto" => Ok(Val::Auto),
            "undefined" => Ok(Val::Undefined),
            _ => Err(ParseError::with_location(
                ParseErrorKind::InvalidToken,
                location,
            )),
        },
        Ok(Token::Percentage(value)) => Ok(Val::Percent(value)),
        Err(e) => Err(e),
        _ => Err(ParseError::with_location(
            ParseErrorKind::UnexpectedToken,
            location,
        )),
    }
}

pub fn parse_color<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Color, ParseError<'i, BasicParseErrorHandler<'i>>> {
    let location = input.current_source_location();
    match input.next() {
        Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            "white" => Ok(Color::WHITE),
            "black" => Ok(Color::BLACK),
            "transparent" => Ok(Color::NONE),
            _ => Err(ParseError::with_location(
                ParseErrorKind::InvalidToken,
                location,
            )),
        },
        Ok(Token::Hash(hash)) => {
            let hex = hash.as_str();
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
                Ok(Color::rgb(
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                ))
            } else if hex.len() == 8 {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
                let a = u8::from_str_radix(&hex[6..8], 16).unwrap();
                Ok(Color::rgba(
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    a as f32 / 255.0,
                ))
            } else {
                Err(ParseError::with_location(
                    ParseErrorKind::InvalidToken,
                    location,
                ))
            }
        }
        Err(e) => Err(e),
        _ => Err(ParseError::with_location(
            ParseErrorKind::UnexpectedToken,
            location,
        )),
    }
}

pub fn parse_ui_rect<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<UiRect<Val>, ParseError<'i, BasicParseErrorHandler<'i>>> {
    let location = input.current_source_location();
    let first = parse_value(input)?;
    let second = input.try_parse(parse_value)?;
    let third = input.try_parse(parse_value)?;
    let fourth = input.try_parse(parse_value)?;

    match (second, third, fourth) {
        (None, None, None) => Ok(UiRect::all(first)),
        (Some(second), None, None) => Ok(UiRect::new(first, second, first, second)),
        (Some(second), Some(third), None) => Ok(UiRect::new(first, second, third, second)),
        (Some(second), Some(third), Some(fourth)) => Ok(UiRect {
            left: fourth,
            right: second,
            top: first,
            bottom: third,
        }),
        _ => Err(ParseError::with_location(
            ParseErrorKind::UnexpectedToken,
            location,
        )),
    }
}

pub fn parse_rule<'i, 't>(
    input: &mut Parser<'i, 't>,
    selector: &str,
    rules: &mut HashMap<String, StyleProperties>,
) -> Result<(), ParseError<'i, BasicParseErrorHandler<'i>>> {
    let location = input.current_source_location();
    match input.next() {
        Ok(Token::Ident(property_name)) => match input.next() {
            Ok(Token::Colon) => {
                let mut style_props = rules
                    .entry(selector.to_string())
                    .or_insert_with(StyleProperties::default);
                match property_name.to_lowercase().as_str() {
                    "display" => {
                        let location = input.current_source_location();
                        let display_value = match input.next() {
                            Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
                                "none" => Ok(Display::None),
                                "flex" => Ok(Display::Flex),
                                _ => Err(ParseError::with_location(
                                    ParseErrorKind::InvalidToken,
                                    location,
                                )),
                            },
                            Err(e) => Err(e),
                            _ => Err(ParseError::with_location(
                                ParseErrorKind::UnexpectedToken,
                                location,
                            )),
                        }?;
                        style_props.display = Some(display_value);
                    }
                    "position-type" => {
                        let location = input.current_source_location();
                        let position_type_value = match input.next() {
                            Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
                                "absolute" => Ok(PositionType::Absolute),
                                "relative" => Ok(PositionType::Relative),
                                _ => Err(ParseError::with_location(
                                    ParseErrorKind::InvalidToken,
                                    location,
                                )),
                            },
                            Err(e) => Err(e),
                            _ => Err(ParseError::with_location(
                                ParseErrorKind::UnexpectedToken,
                                location,
                            )),
                        }?;
                        style_props.position_type = Some(position_type_value);
                    }
                    "flex-direction" => {
                        let location = input.current_source_location();
                        let flex_direction_value = match input.next() {
                            Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
                                "row" => Ok(FlexDirection::Row),
                                "column" => Ok(FlexDirection::Column),
                                "row-reverse" => Ok(FlexDirection::RowReverse),
                                "column-reverse" => Ok(FlexDirection::ColumnReverse),
                                _ => Err(ParseError::with_location(
                                    ParseErrorKind::InvalidToken,
                                    location,
                                )),
                            },
                            Err(e) => Err(e),
                            _ => Err(ParseError::with_location(
                                ParseErrorKind::UnexpectedToken,
                                location,
                            )),
                        }?;
                        style_props.flex_direction = Some(flex_direction_value);
                    }
                    "flex-wrap" => {
                        let location = input.current_source_location();
                        let flex_wrap_value = match input.next() {
                            Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
                                "nowrap" => Ok(FlexWrap::NoWrap),
                                "wrap" => Ok(FlexWrap::Wrap),
                                "wrap-reverse" => Ok(FlexWrap::WrapReverse),
                                _ => Err(ParseError::with_location(
                                    ParseErrorKind::InvalidToken,
                                    location,
                                )),
                            },
                            Err(e) => Err(e),
                            _ => Err(ParseError::with_location(
                                ParseErrorKind::UnexpectedToken,
                                location,
                            )),
                        }?;
                        style_props.flex_wrap = Some(flex_wrap_value);
                    }
                    "align-items" => {
                        let location = input.current_source_location();
                        let align_items_value = match input.next() {
                            Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
                                "flex-start" => Ok(AlignItems::FlexStart),
                                "flex-end" => Ok(AlignItems::FlexEnd),
                                "center" => Ok(AlignItems::Center),
                                "baseline" => Ok(AlignItems::Baseline),
                                "stretch" => Ok(AlignItems::Stretch),
                                _ => Err(ParseError::with_location(
                                    ParseErrorKind::InvalidToken,
                                    location,
                                )),
                            },
                            Err(e) => Err(e),
                            _ => Err(ParseError::with_location(
                                ParseErrorKind::UnexpectedToken,
                                location,
                            )),
                        }?;
                        style_props.align_items = Some(align_items_value);
                    }
                    "align-self" => {
                        let location = input.current_source_location();
                        let align_self_value = match input.next() {
                            Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
                                "auto" => Ok(AlignSelf::Auto),
                                "flex-start" => Ok(AlignSelf::FlexStart),
                                "flex-end" => Ok(AlignSelf::FlexEnd),
                                "center" => Ok(AlignSelf::Center),
                                "baseline" => Ok(AlignSelf::Baseline),
                                "stretch" => Ok(AlignSelf::Stretch),
                                _ => Err(ParseError::with_location(
                                    ParseErrorKind::InvalidToken,
                                    location,
                                )),
                            },
                            Err(e) => Err(e),
                            _ => Err(ParseError::with_location(
                                ParseErrorKind::UnexpectedToken,
                                location,
                            )),
                        }?;
                        style_props.align_self = Some(align_self_value);
                    }
                    "align-content" => {
                        let location = input.current_source_location();
                        let align_content_value = match input.next() {
                            Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
                                "flex-start" => Ok(AlignContent::FlexStart),
                                "flex-end" => Ok(AlignContent::FlexEnd),
                                "center" => Ok(AlignContent::Center),
                                "space-between" => Ok(AlignContent::SpaceBetween),
                                "space-around" => Ok(AlignContent::SpaceAround),
                                "stretch" => Ok(AlignContent::Stretch),
                                _ => Err(ParseError::with_location(
                                    ParseErrorKind::InvalidToken,
                                    location,
                                )),
                            },
                            Err(e) => Err(e),
                            _ => Err(ParseError::with_location(
                                ParseErrorKind::UnexpectedToken,
                                location,
                            )),
                        }?;
                        style_props.align_content = Some(align_content_value);
                    }
                    "justify-content" => {
                        let location = input.current_source_location();
                        let justify_content_value = match input.next() {
                            Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
                                "flex-start" => Ok(JustifyContent::FlexStart),
                                "flex-end" => Ok(JustifyContent::FlexEnd),
                                "center" => Ok(JustifyContent::Center),
                                "space-between" => Ok(JustifyContent::SpaceBetween),
                                "space-around" => Ok(JustifyContent::SpaceAround),
                                "space-evenly" => Ok(JustifyContent::SpaceEvenly),
                                _ => Err(ParseError::with_location(
                                    ParseErrorKind::InvalidToken,
                                    location,
                                )),
                            },
                            Err(e) => Err(e),
                            _ => Err(ParseError::with_location(
                                ParseErrorKind::UnexpectedToken,
                                location,
                            )),
                        }?;
                        style_props.justify_content = Some(justify_content_value);
                    }
                    "position" => {
                        style_props.position = parse_ui_rect(input)?;
                    }
                    "margin" => {
                        style_props.margin = parse_ui_rect(input)?;
                    }
                    "padding" => {
                        style_props.padding = parse_ui_rect(input)?;
                    }
                    "border" => {
                        style_props.border = parse_ui_rect(input)?;
                    }
                    "width" => {
                        style_props.width = Some(parse_value(input)?);
                    }
                    "height" => {
                        style_props.height = Some(parse_value(input)?);
                    }
                    "min-width" => {
                        style_props.min_width = Some(parse_value(input)?);
                    }
                    "max-width" => {
                        style_props.max_width = Some(parse_value(input)?);
                    }
                    "min-height" => {
                        style_props.min_height = Some(parse_value(input)?);
                    }
                    "max-height" => {
                        style_props.max_height = Some(parse_value(input)?);
                    }
                    "aspect-ratio" => {
                        let location = input.current_source_location();
                        match input.next() {
                            Ok(Token::Number { value, .. }) => {
                                style_props.aspect_ratio = Some(value);
                            }
                            Err(e) => return Err(e),
                            _ => {
                                return Err(ParseError::with_location(
                                    ParseErrorKind::UnexpectedToken,
                                    location,
                                ));
                            }
                        }
                    }
                    "overflow" => {
                        let location = input.current_source_location();
                        let overflow_value = match input.next() {
                            Ok(Token::Ident(ident)) => match ident.to_lowercase().as_str() {
                                "visible" => Ok(Overflow::Visible),
                                "hidden" => Ok(Overflow::Hidden),
                                "scroll" => Ok(Overflow::Scroll),
                                _ => Err(ParseError::with_location(
                                    ParseErrorKind::InvalidToken,
                                    location,
                                )),
                            },
                            Err(e) => Err(e),
                            _ => Err(ParseError::with_location(
                                ParseErrorKind::UnexpectedToken,
                                location,
                            )),
                        }?;
                        style_props.overflow = Some(overflow_value);
                    }
                    "background-color" => {
                        let color = parse_color(input)?;
                        style_props.background_color = Some(BackgroundColor(color));
                    }
                    "custom-property" => {
                        let location = input.current_source_location();
                        match input.next() {
                            Ok(Token::Ident(ident)) => {
                                style_props.custom_property = Some(ident.to_string());
                            }
                            Ok(Token::QuotedString(string)) => {
                                style_props.custom_property = Some(string.to_string());
                            }
                            Err(e) => return Err(e),
                            _ => {
                                return Err(ParseError::with_location(
                                    ParseErrorKind::UnexpectedToken,
                                    location,
                                ));
                            }
                        }
                    }
                    _ => {
                        while let Ok(token) = input.next() {
                            if token == Token::Semicolon {
                                break;
                            }
                        }
                    }
                }
            }
            Ok(Token::CloseCurlyBrace) => {
                return Ok(());
            }
            Err(e) => return Err(e),
            _ => {
                return Err(ParseError::with_location(
                    ParseErrorKind::UnexpectedToken,
                    location,
                ));
            }
        },
        Ok(Token::CloseCurlyBrace) => {
            return Ok(());
        }
        Err(e) => return Err(e),
        _ => {
            return Err(ParseError::with_location(
                ParseErrorKind::UnexpectedToken,
                location,
            ));
        }
    }
    Ok(())
}

pub fn parse_stylesheet(css: &str, rules: &mut HashMap<String, StyleProperties>) {
    let mut input = Parser::new(css, BasicParseErrorHandler::new());

    while let Ok(token) = input.next() {
        match token {
            Token::StartRule { .. } => {
                let selector = match input.next() {
                    Ok(Token::Ident(selector)) | Ok(Token::QuotedString(selector)) => {
                        selector.to_string()
                    }
                    Ok(Token::Colon) => {
                        let selector_name = match input.next() {
                            Ok(Token::Ident(selector_name)) => selector_name.to_string(),
                            Ok(Token::QuotedString(selector_name)) => selector_name.to_string(),
                            _ => continue,
                        };
                        ":".to_string() + &selector_name
                    }
                    _ => {
                        continue;
                    }
                };

                while let Ok(token) = input.next() {
                    match token {
                        Token::CloseCurlyBrace => {
                            break;
                        }
                        Token::Semicolon => {}
                        _ => {
                            input.reset_to(token);
                            if let Err(e) = parse_rule(&mut input, &selector, rules) {
                                match e.kind {
                                    ParseErrorKind::UnexpectedToken => {
                                        while let Ok(t) = input.next() {
                                            if t == Token::Semicolon || t == Token::CloseCurlyBrace
                                            {
                                                break;
                                            }
                                        }
                                    }
                                    ParseErrorKind::InvalidToken => {
                                        while let Ok(t) = input.next() {
                                            if t == Token::Semicolon || t == Token::CloseCurlyBrace
                                            {
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        println!("Parse Error: {:?}", e);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Token::AtKeyword(_) => {
                while let Ok(token) = input.next() {
                    if token == Token::CloseCurlyBrace {
                        break;
                    }
                }
            }
            Token::Whitespace => {}
            Token::CloseCurlyBrace => {}
            Token::Semicolon => {}
            _ => {}
        }
    }
}
