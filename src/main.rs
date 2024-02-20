use bevy::prelude::*;
use space_editor::prelude::*;

mod level;

#[cfg(feature = "editor")]
fn init_editor(app: &mut App) {
    app.add_plugins(SpaceEditorPlugin)
        .add_systems(Startup, simple_editor_setup);
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugins(PrefabPlugin);

    #[cfg(feature = "editor")]
    init_editor(&mut app);

    #[cfg(not(feature = "editor"))]
    app.add_systems(Startup, level::load_level);

    app.run();
}
