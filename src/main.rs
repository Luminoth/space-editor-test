use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use space_editor::prelude::*;

mod editor;
mod scene;

#[cfg(not(feature = "editor"))]
fn init_game(app: &mut App) {
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugins(space_editor::space_prefab::plugins::PrefabPlugin)
        .add_systems(Startup, scene::load_level);
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Space Editor Test".into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default());

    #[cfg(feature = "editor")]
    editor::init(&mut app);

    #[cfg(not(feature = "editor"))]
    init_game(&mut app);

    app.editor_registry::<RigidBody>()
        //.editor_registry::<Collider>()
        .editor_registry::<Restitution>();

    app.run();
}
