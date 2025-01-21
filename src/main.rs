pub mod grid;
pub mod display_material;

use bevy::{color, prelude::*, render::{render_resource::{BufferInitDescriptor, BufferUsages}, renderer::RenderDevice}, window::PresentMode};
use display_material::DisplayMaterial;
use grid::{hue_shift, set_direction, update_cells, update_cells_display, Cell, Cells, SnakeHead, SnakeHeadDirection};

fn main() {
    App::new().
    add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Snake Game".into(),
            name: Some("Snake Game".into()),
            resolution: (512., 512.).into(),
            present_mode: PresentMode::AutoVsync,
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    })).
    add_plugins(MaterialPlugin::<DisplayMaterial>::default()).
    insert_resource(SnakeHeadDirection {
        x:0,
        y:1
    }).
    insert_resource(ClearColor(Color::linear_rgb(0.0, 0.0, 0.0))).
    insert_resource(Time::<Fixed>::from_hz(10.0)).
    add_systems(Startup, camera_setup).
    add_systems(Startup, display_setup).
    add_systems(Startup, game_setup).
    add_systems(Update, set_direction).
    add_systems(FixedUpdate, (update_cells,update_cells_display)).
    run();
}

fn camera_setup(mut commands:Commands) {
    commands.spawn(Camera3dBundle {
        transform:Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

const BOARD_X:i32 = 64;
const BOARD_Y:i32 = 64;

fn display_setup(mut commands:Commands,mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<DisplayMaterial>>) {
    commands.spawn(MaterialMeshBundle {
        mesh:meshes.add(Rectangle::from_size(Vec2::new(0.82, 0.82))),
        transform:Transform::from_xyz(0.0, 0.0, -1.0),
        material:materials.add(DisplayMaterial{
            cells:vec![0,1,0,0],
            size_x:2,
            size_y:2,
            background_color:LinearRgba::BLACK,
            snake_color:LinearRgba::WHITE,
            food_color:LinearRgba::WHITE
        }),
        ..default()
    });
}

fn game_setup(mut commands:Commands) {
    let mut cells:Vec<Cell> = Vec::new();

    for x in 0..BOARD_X {
        for y in 0..BOARD_Y {
            cells.push(Cell{
                cell_x_index:x,
                cell_y_index:y,
                state:grid::CellState::Empty,
                snake_body_lifetime:0
            });
        }
    }

    let mut cells = Cells
    {
        size_x:BOARD_X,
        size_y:BOARD_Y,
        cells:cells
    };
    cells.add_food();
    commands.spawn(cells);

    commands.spawn(SnakeHead {
        x:BOARD_X/2,
        y:BOARD_Y/2,
        snake_size:6
    });
}