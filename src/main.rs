use bevy::prelude::*;

const PLAYER_A_SPRITE: &str = "player_a.png";
const TIME_STEP: f32 = 1.0 / 60.0;

// ### Resources ###

struct Materials {
    player_material: Handle<ColorMaterial>,
}

// ### Components ###

struct Player;
struct PlayerSpeed(f32);

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Basics: Space Invaders!".to_owned(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
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
        player_material: materials.add(asset_server.load(PLAYER_A_SPRITE).into()),
    });

    window.set_position(IVec2::new(750, 200));
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>) {
    let bottom = -300.0;
    let material = materials.player_material.clone();
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
        .insert(PlayerSpeed(500.0));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>,
) {
    if let Ok((PlayerSpeed(speed), mut transform, _)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };
        transform.translation.x += dir * speed * TIME_STEP;
    }
}
