use std::f32::consts::PI;

use bevy::{asset::{Assets, Handle}, color::{Color, LinearRgba}, input::ButtonInput, prelude::{Component, KeyCode, Query, Res, ResMut, Resource}, time::Time};
use rand::prelude::*;

use crate::display_material::DisplayMaterial;

#[derive(Debug,PartialEq)]
pub enum CellState {
    SnakeBody,
    Food,
    Empty
}

pub struct Cell
{
    pub cell_x_index:i32,
    pub cell_y_index:i32,
    pub state:CellState,
    pub snake_body_lifetime:u32
}

#[derive(Component)]
pub struct Cells
{
    pub cells:Vec<Cell>,
    pub size_x:i32,
    pub size_y:i32
}

impl Cells {
    fn get_cell(&mut self,x:i32,y:i32) -> &mut Cell {
        &mut self.cells[(x + y * self.size_x) as usize]
    }
    
    pub fn add_food(&mut self) {
        let mut free_cells:Vec<&mut Cell> = self.cells.iter_mut().filter(|cell|cell.state != CellState::SnakeBody).collect();
        let index = rand::thread_rng().gen_range(0..free_cells.len());
        let food_cell = free_cells.get_mut(index).unwrap();
        food_cell.state = CellState::Food;
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.state = CellState::Empty;
        }
        self.add_food();
    }
}

#[derive(Component)]
pub struct SnakeHead
{
    pub x:i32,
    pub y:i32,
    pub snake_size:u32
}

#[derive(Resource)]
pub struct SnakeHeadDirection
{
    pub x:i32,
    pub y:i32
}

pub fn set_direction(mut direction:ResMut<SnakeHeadDirection>,key:Res<ButtonInput<KeyCode>>,mut cells_query:Query<&mut Cells>,mut snake_head:Query<&mut SnakeHead>) {
    if key.just_pressed(KeyCode::KeyF) {
        cells_query.single_mut().add_food();
    }

    if key.just_pressed(KeyCode::Equal)
    {
        snake_head.single_mut().snake_size += 1;
    }

    if direction.x != 0
    {
        if key.just_pressed(KeyCode::ArrowUp) {
            direction.x = 0;
            direction.y = -1;
            return;
        }
        if key.just_pressed(KeyCode::ArrowDown) {
            direction.x = 0;
            direction.y = 1;
            return;
        }
    }
    if direction.y == 0
    {
        return;
    }
    if key.just_pressed(KeyCode::ArrowLeft) {
        direction.x = -1;
        direction.y = 0;
        return;
    }
    if key.just_pressed(KeyCode::ArrowRight) {
        direction.x = 1;
        direction.y = 0;
        return;
    }  
}

pub fn hue_shift(mut materials:ResMut<Assets<DisplayMaterial>>,query: Query<&Handle<DisplayMaterial>>,time:Res<Time>)
{
    for handle in query.iter(){
        if let Some(mat) = materials.get_mut(handle){
            mat.background_color = Color::hsv(time.elapsed_seconds().sin() * 0.5 + 1.0, 1.0, 0.7).into();
            mat.snake_color = Color::hsv((time.elapsed_seconds() + PI * 0.7).sin() * 0.5 + 0.5, 1.0, 0.7).into();
            mat.background_color = Color::hsv((time.elapsed_seconds() + PI * 1.4).sin() * 0.5 + 0.5, 1.0, 0.7).into();
        }
    }
}

pub fn update_cells_display(cells_query:Query<&Cells>,mut materials:ResMut<Assets<DisplayMaterial>>,query: Query<&Handle<DisplayMaterial>>) {
    let cells = cells_query.single();
    for handle in query.iter(){
        if let Some(mat) = materials.get_mut(handle){
            mat.size_x = cells.size_x as u32;
            mat.size_y = cells.size_y as u32;
            mat.cells = cells.cells.iter()
            .map(|cell|match cell.state {
                CellState::Empty => 0,
                CellState::SnakeBody => 1,
                CellState::Food => 2
            })
            .collect();
        }
    }
}

pub fn update_cells(mut cells_query:Query<&mut Cells>,mut head:Query<&mut SnakeHead>,mut direction:ResMut<SnakeHeadDirection>) {
    let mut cells = cells_query.single_mut();
    let mut head = head.single_mut();

    for cell in cells.cells.iter_mut() {
        match cell.state {
            CellState::Empty | CellState::Food => continue,
            CellState::SnakeBody => {
                cell.snake_body_lifetime += 1;
                if cell.snake_body_lifetime > head.snake_size {
                    cell.state = CellState::Empty
                }
            }
        }
    }

    head.x += direction.x;
    head.y += direction.y;

    if head.x < 0 || head.x >= cells.size_x || head.y < 0 || head.y >= cells.size_y
    {
        cells.clear();
        head.x = cells.size_x / 2;
        head.y = cells.size_y / 2;
        head.snake_size = 6;
        direction.x = 0;
        direction.y = 1;
    }

    let target_cell = cells.get_cell(head.x, head.y);
    match target_cell.state {
        CellState::Empty => {
            target_cell.state = CellState::SnakeBody;
            target_cell.snake_body_lifetime = 0;
        },
        CellState::Food => {
            target_cell.state = CellState::SnakeBody;
            head.snake_size += 1;
            target_cell.snake_body_lifetime = 0;
            cells.add_food();
        }
        CellState::SnakeBody => {
            cells.clear();
            head.x = cells.size_x / 2;
            head.y = cells.size_y / 2;
            head.snake_size = 6;
            direction.x = 0;
            direction.y = 1;
        }
    }
}