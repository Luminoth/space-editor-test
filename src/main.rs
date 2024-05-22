use bevy::prelude::*;

mod editor;
mod scene;

#[cfg(not(feature = "editor"))]
fn init_game(app: &mut App) {
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugins(space_editor::space_prefab::plugins::PrefabPlugin)
        .add_systems(Startup, scene::load_level);

    #[cfg(not(feature = "rapier"))]
    {
        app.add_plugins(space_editor::space_bevy_xpbd_plugin::XpbdPlugin);
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Space Editor Test".into(),
            ..default()
        }),
        ..default()
    }));

    #[cfg(feature = "rapier")]
    {
        use bevy_rapier3d::prelude::*;

        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default());
    }

    editor::init(&mut app);

    #[cfg(not(feature = "editor"))]
    init_game(&mut app);

    app.run();
}
