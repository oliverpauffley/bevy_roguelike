use crate::vectors::Vector2Int;
use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub v: Vector2Int,
}

#[derive(Component)]
pub struct Tile;
