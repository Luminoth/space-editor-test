use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use space_editor::prelude::*;

mod level;

#[cfg(feature = "editor")]
fn init_editor(app: &mut App) {
    app.add_plugins(SpaceEditorPlugin)
        .add_systems(Startup, simple_editor_setup);
}

#[cfg(not(feature = "editor"))]
fn init_game(app: &mut App) {
    app.add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, level::load_level);
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugins(PrefabPlugin);

    #[cfg(feature = "editor")]
    init_editor(&mut app);

    #[cfg(not(feature = "editor"))]
    init_game(&mut app);

    app.run();
}
