#![cfg(feature = "editor")]

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use space_editor::prelude::*;

pub fn init(app: &mut App) {
    app.add_plugins(SpaceEditorPlugin)
        .add_systems(Startup, simple_editor_setup);

    // TODO: we need to disable physics when not running

    app.editor_bundle(
        "Rapier",
        "Static Box",
        (
            MeshPrimitive3dPrefab::Box(BoxPrefab::default()),
            Name::new("Static Box".to_string()),
            RigidBody::Fixed,
            Collider::cuboid(0.5, 0.5, 0.5),
            Transform::default(),
            Visibility::default(),
        ),
    );

    app.editor_bundle(
        "Rapier",
        "Dynamic Sphere",
        (
            MeshPrimitive3dPrefab::Sphere(SpherePrefab::default()),
            Name::new("Dynamic Sphere".to_string()),
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Transform::default(),
            Visibility::default(),
        ),
    );
}
