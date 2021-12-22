use bevy::prelude::*;

mod consts;
mod player;

use consts::*;
use player::PlayerPlugin;

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
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
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
