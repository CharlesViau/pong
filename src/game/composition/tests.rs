use avian2d::prelude::{Collider, LinearVelocity, Position, RigidBody};
use bevy::prelude::*;
use pong_core::{Side, rules};

use super::{ball_factory::BallFactory, paddle_factory::PaddleFactory};
use crate::game::{
    infrastructure::bevy::{BallBody, PaddleBody},
    presentation::SceneAssets,
};

#[test]
fn spawning_a_paddle_creates_its_physics_and_visual_without_game_startup() {
    for side in [Side::Left, Side::Right] {
        // Given an empty world with shared visual assets, without the game's startup systems.
        let (mut world, assets) = factory_world();
        let expected_position = Vec2::new(side.sign() * rules::PADDLE_X, 0.0);
        let expected_material = match side {
            Side::Left => &assets.left,
            Side::Right => &assets.right,
        };

        // When the factory's queued commands are applied.
        let paddle = PaddleFactory::spawn(&mut world.commands(), &assets, side);
        world.flush();

        // Then the returned entity is a physical paddle with its correctly sized visual.
        assert!(world.get::<PaddleBody>(paddle).is_some());
        assert_eq!(world.get::<RigidBody>(paddle), Some(&RigidBody::Kinematic));
        assert!(world.get::<Collider>(paddle).is_some());
        assert_eq!(world.get::<Position>(paddle).unwrap().0, expected_position);
        assert_eq!(world.get::<LinearVelocity>(paddle).unwrap().0, Vec2::ZERO);
        assert_visual_child(
            &world,
            paddle,
            Vec2::new(rules::PADDLE_WIDTH, rules::PADDLE_HEIGHT),
            expected_material,
        );
    }
}

#[test]
fn spawning_a_ball_creates_its_physics_and_visual_without_game_startup() {
    // Given an empty world with shared visual assets, without the game's startup systems.
    let (mut world, assets) = factory_world();

    // When the factory's queued commands are applied.
    let ball = BallFactory::spawn(&mut world.commands(), &assets);
    world.flush();

    // Then the ball starts at rest in the center with a visual attached to its physics body.
    assert!(world.get::<BallBody>(ball).is_some());
    assert_eq!(world.get::<RigidBody>(ball), Some(&RigidBody::Dynamic));
    assert!(world.get::<Collider>(ball).is_some());
    assert_eq!(world.get::<Position>(ball).unwrap().0, Vec2::ZERO);
    assert_eq!(world.get::<LinearVelocity>(ball).unwrap().0, Vec2::ZERO);
    assert_visual_child(
        &world,
        ball,
        Vec2::splat(rules::BALL_RADIUS * 2.0),
        &assets.ball,
    );
}

fn factory_world() -> (World, SceneAssets) {
    let mut world = World::new();
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<ColorMaterial>::default();
    let assets = SceneAssets::new(&mut meshes, &mut materials);
    world.insert_resource(meshes);
    world.insert_resource(materials);
    (world, assets)
}

#[track_caller]
fn assert_visual_child(
    world: &World,
    body: Entity,
    expected_size: Vec2,
    expected_material: &Handle<ColorMaterial>,
) {
    let transform = world.get::<Transform>(body).unwrap();
    assert_eq!(
        transform.scale,
        Vec3::ONE,
        "physics root must stay unscaled"
    );
    assert_eq!(
        transform.translation.truncate(),
        world.get::<Position>(body).unwrap().0,
        "initial rendering must match the physics position"
    );

    let children = world.get::<Children>(body).expect("body needs a visual");
    assert_eq!(children.len(), 1, "body has one visual child");
    let visual = world.entity(children[0]);
    let visual_transform = visual.get::<Transform>().unwrap();
    assert_eq!(visual_transform.translation, Vec3::ZERO);
    assert_eq!(visual_transform.scale, expected_size.extend(1.0));
    let material = visual.get::<MeshMaterial2d<ColorMaterial>>().unwrap();
    assert_eq!(&material.0, expected_material);
    let mesh = visual.get::<Mesh2d>().unwrap();
    assert!(world.resource::<Assets<Mesh>>().get(&mesh.0).is_some());
}
