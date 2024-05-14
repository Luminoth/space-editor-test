#![cfg(not(feature = "editor"))]

use bevy::prelude::*;
use space_editor::prelude::*;

pub fn load_level(mut commands: Commands) {
    info!("loading level ...");

    let mut level = if cfg!(feature = "rapier") {
        commands.spawn(PrefabBundle::new("scenes/Scene0-rapier.scn.ron"))
    } else {
        commands.spawn(PrefabBundle::new("scenes/Scene0.scn.ron"))
    };
    level.insert(Name::new("Level"));

    // idk why tf but cameras in the scene aren't working
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
