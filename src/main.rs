use bevy::prelude::*;

const PLAYER_A_SPRITE: &str = "player_a.png";

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
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(750, 200));

    let bottom = -window.height() / 2.0;
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load(PLAYER_A_SPRITE).into()),
        transform: Transform {
            translation: Vec3::new(0.0, bottom + 30.0, 10.0),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
