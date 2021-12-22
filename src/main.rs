use bevy::prelude::*;

const PLAYER_A_SPRITE: &str = "player_a.png";
const LASER_A_SPRITE: &str = "laser_a.png";
const TIME_STEP: f32 = 1.0 / 60.0;
const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 600.0;

// ### Resources ###

struct Materials {
    player: Handle<ColorMaterial>,
    laser: Handle<ColorMaterial>,
}

// ### Components ###

struct Player;
struct PlayerReadyFire(bool);
struct Speed(f32);
struct Laser;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Basics: Space Invaders!".to_owned(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .add_system(player_fire.system())
        .add_system(laser_movement.system())
        .run();
}

// ## Systems ##

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // TODO: Error handling
    let window = windows.get_primary_mut().unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(Materials {
        player: materials.add(asset_server.load(PLAYER_A_SPRITE).into()),
        laser: materials.add(asset_server.load(LASER_A_SPRITE).into()),
    });

    window.set_position(IVec2::new(750, 200));
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>) {
    let bottom = -WINDOW_HEIGHT / 2.0;
    let material = materials.player.clone();
    let transform = Transform {
        translation: Vec3::new(0.0, bottom + 25.0, 10.0),
        scale: Vec3::new(0.5, 0.5, 1.0),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            material,
            transform,
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(500.0))
        .insert(PlayerReadyFire(true));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>,
) {
    if let Ok((Speed(speed), mut transform, _)) = query.single_mut() {
        let dir = if keyboard.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };
        transform.translation.x += dir * speed * TIME_STEP;
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&Transform, &mut PlayerReadyFire, With<Player>)>,
) {
    if let Ok((&player_transform, mut ready_fire, _)) = query.single_mut() {
        let PlayerReadyFire(ref mut ready_fire) = *ready_fire;
        if *ready_fire && keyboard.pressed(KeyCode::Space) {
            let x = player_transform.translation.x;
            let y = player_transform.translation.y;
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        scale: Vec3::new(0.5, 0.5, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(Speed(500.0));
            *ready_fire = false;
        }
        if keyboard.just_released(KeyCode::Space) {
            *ready_fire = true;
        }
    }
}

fn laser_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Speed, &mut Transform, With<Laser>)>,
) {
    for (laser_entity, Speed(speed), mut laser_transform, _) in query.iter_mut() {
        let translation = &mut laser_transform.translation;
        translation.y += speed * TIME_STEP;
        if translation.y > WINDOW_HEIGHT {
            commands.entity(laser_entity).despawn();
        }
    }
}
