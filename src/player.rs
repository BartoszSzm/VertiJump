use crate::world::World;
use animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;

use crate::extensions::Square;
use std::collections::VecDeque;

pub enum Direction {
    Left,
    Right,
    Down,
    None,
}

pub struct Player {
    pub rect: Rect,
    pub default_speed: Vec2,
    pub texture: Texture2D,
    pub sprite: AnimatedSprite,
    pub is_falling: bool,
    pub speed: Vec2,
    pub world: World,
    positions_y: VecDeque<f32>,
    fall_time: f32,
    max_fall_time: f32,
}

impl Player {
    pub async fn new(
        texture_path: &str,
        default_speed: Vec2,
        player_tile_dims: &Square<u32>,
        spawn_point: &Vec2,
        world: World,
    ) -> Player {
        let sprite = Player::load_player_sprite(player_tile_dims.width, player_tile_dims.height);

        // TODO Create collisions in Tiled
        let mut rect = Rect::new(
            0.,
            0.,
            sprite.frame().source_rect.w - 5.,
            sprite.frame().source_rect.h + 4.,
        );
        rect.move_to(*spawn_point);
        let texture = Player::load_player_texture(&texture_path).await;
        let is_falling = false;
        let positions_y = VecDeque::from([rect.y, 0.0, 0.0]);
        let speed = default_speed;
        let fall_time = 0.;
        let max_fall_time = 5.;
        Player {
            rect,
            default_speed,
            texture,
            sprite,
            world,
            is_falling,
            positions_y,
            speed,
            fall_time,
            max_fall_time,
        }
    }

    async fn load_player_texture(texture_path: &str) -> Texture2D {
        let texture = load_texture(texture_path)
            .await
            .expect("Cannot load player texture");
        texture.set_filter(FilterMode::Nearest);
        texture
    }

    fn load_player_sprite(player_tile_width: u32, player_tile_height: u32) -> AnimatedSprite {
        AnimatedSprite::new(
            player_tile_width,
            player_tile_height,
            &[
                Animation {
                    name: "nomove".to_string(),
                    row: 0,
                    frames: 10,
                    fps: 5,
                },
                Animation {
                    name: "left".to_string(),
                    row: 1,
                    frames: 10,
                    fps: 5,
                },
                Animation {
                    name: "right".to_string(),
                    row: 2,
                    frames: 10,
                    fps: 5,
                },
                Animation {
                    name: "fall".to_string(),
                    row: 3,
                    frames: 10,
                    fps: 5,
                },
            ],
            true,
        )
    }

    pub fn update(&mut self) {
        // Check if player is falling
        if let Some(&previous_position) = self.positions_y.front() {
            if previous_position != self.rect.y {
                self.is_falling = true;
                self.speed.x = self.default_speed.x + 1.;
                if self.fall_time <= self.max_fall_time {
                    self.fall_time += 0.02;
                }
            } else {
                self.is_falling = false;
                self.speed.x = self.default_speed.x;
                self.fall_time = 0.;
            }
        }

        // Push current y pos to the front, remove last element
        self.positions_y.push_front(self.rect.y);
        self.positions_y.pop_back();

        self.apply_gravity();
    }

    pub fn _move(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                let movement_vector = vec2(-self.speed.x, 0.);
                if !self.world.collision(&self.rect, movement_vector) {
                    self.sprite.set_animation(2);
                    self.rect.move_to(self.rect.point() + movement_vector);
                }
            }
            Direction::Right => {
                let movement_vector = vec2(self.speed.x, 0.);
                if !self.world.collision(&self.rect, movement_vector) {
                    self.sprite.set_animation(1);
                    self.rect.move_to(self.rect.point() + movement_vector);
                }
            }
            Direction::Down => {
                let movement_vector = vec2(0., self.speed.y);
                if !self.world.collision(&self.rect, movement_vector) {
                    self.sprite.set_animation(3);
                    self.rect.move_to(self.rect.point() + movement_vector);
                }
            }
            Direction::None => {
                self.sprite.set_animation(0);
            }
        }
        self.sprite.update();
    }

    pub fn jump(&mut self, height: f32) {
        if !self.is_falling {
            self.speed.y = -height;
        }
    }

    fn apply_gravity(&mut self) {
        if self.world.collision(&self.rect, vec2(0., self.speed.y)) {
            self.speed.y = self.default_speed.y;
        }

        let acceleration = 1.5;
        if self.speed.y < 0. {
            self.speed.y = self.speed.y + acceleration * self.fall_time as f32;
        } else {
            self.speed.y = self.default_speed.y + acceleration * self.fall_time as f32;
        }
        self._move(Direction::Down);
    }
}
