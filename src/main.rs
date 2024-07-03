use bevy::{animation, prelude::*, render::camera::ScalingMode, window::*};
 
fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins
            //turn off image filtering for pixel style
            .set(ImagePlugin::default_nearest())
            //set window title and size
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Undertale prototype 1".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    resizable: false,
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )
    .insert_resource(ClearColor(Color::BLACK))
    .add_systems(Startup, setup)
    .add_systems(Update, player_movement)
    .run();
}

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PLAYER_SPEED: f32 = 5.0;
const PLAYER_X_SIZE: f32 = 16.0;
const PLAYER_Y_SIZE: f32 = 16.0;

#[derive(Component)]
pub struct Player{
    pub speed: f32,
    pub name: String,
    pub level: i32,
    pub health: i32,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //camera setup

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin{
        min_width: WINDOW_WIDTH,
        min_height: WINDOW_HEIGHT,
    };
    commands.spawn(camera);

    //soul setup
    let souls_sheet_texture: Handle<Image> = asset_server.load("textures/Souls_Sprite_Sheet_Test.png");
    //red
    let red_soul_layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 2, Some(Vec2::new(0.0, 30.0)), Some(Vec2::new(7.0, 6.0)));
    let red_soul_atlas = texture_atlas_layouts.add(red_soul_layout);

    let soul_damage_indices = AnimationIndices {
        first: 0,
        last: 1,
    };

    //player setup
    let player = Player {
        speed: PLAYER_SPEED,
        name: "Frisk".to_string(),
        level: 1,
        health: 20,
    };

    //spawn player
    commands.spawn((
        SpriteSheetBundle{
            texture: souls_sheet_texture.clone(),
            atlas: TextureAtlas{
                layout: red_soul_atlas.clone(),
                index: soul_damage_indices.first,
            },
            ..default()
        },
        soul_damage_indices,
        player,
    ));
}

fn player_movement(
    mut query: Query<(&Player, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (player, mut transform) in &mut query{
        let mut x = 0.0;
        let mut y = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowUp){
            y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown){
            y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft){
            x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight){
            x += 1.0;
        }

        transform.translation.x += x * player.speed;
        transform.translation.y += y * player.speed;
    }
}