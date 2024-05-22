use bevy::prelude::*;

fn register_types(#[allow(unused_variables)] app: &mut App) {
    #[cfg(feature = "rapier")]
    {
        use bevy_rapier3d::prelude::*;
        app.register_type::<CoefficientCombineRule>();
    }
}

#[cfg(feature = "editor")]
#[cfg(feature = "rapier")]
fn init_rapier(app: &mut App) {
    use bevy_rapier3d::prelude::*;
    use space_editor::prelude::*;

    app.editor_registry::<RigidBody>()
        //.editor_registry::<Collider>()
        .editor_registry::<Restitution>();

    // TODO: we need to disable physics when not running

    app.editor_bundle(
        "Physicals",
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
        "Physicals",
        "Dynamic Sphere",
        (
            MeshPrimitive3dPrefab::Sphere(SpherePrefab::default()),
            Name::new("Dynamic Sphere".to_string()),
            RigidBody::Dynamic,
            Collider::ball(1.0),
            Transform::default(),
            Visibility::default(),
        ),
    );
}

#[cfg(feature = "editor")]
#[cfg(not(feature = "rapier"))]
fn init_xpdb(app: &mut App) {
    use space_editor::prelude::*;

    // TODO: we need to disable physics when not running

    app.editor_bundle(
        "Physicals",
        "Static Box",
        (
            MeshPrimitive3dPrefab::Box(BoxPrefab::default()),
            Name::new("Static Box".to_string()),
            RigidBodyPrefab::Static,
            ColliderPrefab::FromPrefabMesh,
            Transform::default(),
            Visibility::default(),
        ),
    );

    app.editor_bundle(
        "Physicals",
        "Dynamic Sphere",
        (
            MeshPrimitive3dPrefab::Sphere(SpherePrefab::default()),
            Name::new("Dynamic Sphere".to_string()),
            RigidBodyPrefab::Dynamic,
            ColliderPrefab::FromPrefabMesh,
            Transform::default(),
            Visibility::default(),
        ),
    );
}

#[cfg(feature = "editor")]
fn init_editor(app: &mut App) {
    use space_editor::prelude::*;

    app.add_plugins(SpaceEditorPlugin)
        .add_systems(Startup, simple_editor_setup);

    #[cfg(feature = "rapier")]
    init_rapier(app);

    #[cfg(not(feature = "rapier"))]
    init_xpdb(app);
}

pub fn init(app: &mut App) {
    register_types(app);

    #[cfg(feature = "editor")]
    init_editor(app);
}
