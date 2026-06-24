use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};
use bevy_rapier2d::prelude::*;
use std::ops::Range;

#[derive(Debug, Resource)]
struct CameraSettings {
    /// Clamp the orthographic camera's scale to this range
    pub orthographic_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub orthographic_zoom_speed: f32,
}

#[derive(Component)]
struct Ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(CameraSettings {
            // In orthographic projections, we specify camera scale relative to a default value of 1,
            // in which one unit in world space corresponds to one pixel.
            orthographic_zoom_range: 0.1..10.0,
            // This value was hand-tuned to ensure that zooming in and out feels smooth but not slow.
            orthographic_zoom_speed: 0.2,
        })
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude)
        .add_systems(Update, (zoom, sensor_events))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn(Camera2d)
        .insert(Projection::from(OrthographicProjection {
            // This is the default value for scale for orthographic projections.
            // To zoom in and out, change this value, rather than `ScalingMode` or the camera's position.
            scale: 1.,
            ..OrthographicProjection::default_2d()
        }))
        .insert(Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y));
}

fn zoom(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    // Usually, you won't need to handle both types of projection,
    // but doing so makes for a more complete example.
    if let Projection::Orthographic(ref mut orthographic) = *camera.into_inner() {
        // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
        let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.orthographic_zoom_speed;
        // When changing scales, logarithmic changes are more intuitive.
        // To get this effect, we add 1 to the delta, so that a delta of 0
        // results in no multiplicative effect, positive values result in a multiplicative increase,
        // and negative values result in multiplicative decreases.
        let multiplicative_zoom = 1. + delta_zoom;

        orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(
            camera_settings.orthographic_zoom_range.start,
            camera_settings.orthographic_zoom_range.end,
        );
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(300.0, 50.0))
        .insert(RigidBody::KinematicPositionBased)
        .insert(
            Transform::from_xyz(100.0, -100.0, 0.0)
                .with_rotation(Quat::from_rotation_z((10.0_f32).to_radians())),
        );

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(300.0, 50.0))
        .insert(RigidBody::KinematicPositionBased)
        .insert(
            Transform::from_xyz(-200.0, -500.0, 0.0)
                .with_rotation(Quat::from_rotation_z((-20.0_f32).to_radians())),
        );

    commands
        .spawn(Collider::cuboid(2000.0, 50.0))
        .insert(Transform::from_xyz(1000.0, -1000.0, 0.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS);

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.3))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0))
        .insert(Velocity::default())
        .insert(Ball);

    commands
        .spawn(RigidBody::Dynamic)
        .insert(GravityScale(2.0))
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.9))
        .insert(Transform::from_xyz(200.0, 400.0, 0.0))
        .insert(Velocity::default())
        .insert(Ball);
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn sensor_events(
    mut collision_events: MessageReader<CollisionEvent>,
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    for event in collision_events.read() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };

        let ball = if balls.contains(*e1) {
            Some(*e1)
        } else if balls.contains(*e2) {
            Some(*e2)
        } else {
            None
        };

        if let Some(ball_entity) = ball
            && let Ok((mut transform, mut velocity)) = balls.get_mut(ball_entity)
        {
            transform.translation = Vec3::new(200.0, 400.0, 0.0);
            velocity.linear = Vec2::ZERO;
            velocity.angular = 0.0;
        }
    }
}
