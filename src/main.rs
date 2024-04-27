use bevy::prelude::*;
use space_editor::prelude::*;

mod scene;

#[cfg(feature = "editor")]
fn init_editor(app: &mut App) {
    app.add_plugins(SpaceEditorPlugin)
        .add_systems(Startup, simple_editor_setup);
}

#[cfg(not(feature = "editor"))]
fn init_game(app: &mut App) {
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugins(PrefabPlugin)
        .add_systems(Startup, scene::load_scene);
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "editor")]
    init_editor(&mut app);

    #[cfg(not(feature = "editor"))]
    init_game(&mut app);

    app.run();
}
