use bevy::prelude::*;
use crate::show_fps::components::FpsText;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn()
        .insert(Name::new("FPS Text"))
        .insert(FpsText)
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.,
                            color: Color::hex("181818").unwrap(),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.,
                            color: Color::hex("181818").unwrap(),
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        });
}
