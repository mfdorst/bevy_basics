mod components;
mod consts;
mod enemy;
mod laser;
mod player;
mod resources;

use bevy::prelude::*;
use consts::*;
use resources::{ActiveEnemies, Materials};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Basics: Space Invaders!".to_owned(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ActiveEnemies(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(laser::LaserPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_startup_system(setup.system())
        .run();
}

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
        enemy: materials.add(asset_server.load(ENEMY_SPRITE).into()),
    });

    window.set_position(IVec2::new(750, 200));
}
