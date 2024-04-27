#![cfg(not(feature = "editor"))]

use bevy::prelude::*;
use space_editor::prelude::*;

pub fn load_scene(mut commands: Commands) {
    commands
        .spawn(PrefabBundle::new("scenes/Scene0.scn.ron"))
        .insert(Name::new("Level"));
}
