use bevy::prelude::*;
use space_editor::prelude::*;

pub fn load_level(mut commands: Commands) {
    commands
        .spawn(PrefabBundle::new("levels/level01.scn.ron"))
        .insert(Name::new("Level"));
}
