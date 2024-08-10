use tiled::{self, LayerType, Map};

use crate::config::Config;
use macroquad::prelude::*;
use std::collections::HashMap;
use tiled::{Loader, ObjectShape};

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum MapLayers {
    Terrain,
    Objects,
    Lava,
}

#[derive(Clone)]
pub struct World {
    pub texture_layers: HashMap<MapLayers, Texture2D>,
    pub rect: Rect,
    pub map: Map,
    pub object_rects: HashMap<String, Vec<Rect>>,
}

impl World {
    pub async fn new(game_config: &Config) -> World {
        // Textures loading
        let mut texture_layers: HashMap<MapLayers, Texture2D> = HashMap::new();

        let terrain = load_texture(&game_config.paths.map_terrain_texture)
            .await
            .unwrap();
        terrain.set_filter(FilterMode::Linear);

        // Textures filtering
        texture_layers.insert(MapLayers::Terrain, terrain.clone());
        texture_layers.insert(MapLayers::Lava, terrain.clone());

        let world_size = vec2(terrain.width(), terrain.height());

        // Load map file
        let map = Loader::new()
            .load_tmx_map(&game_config.paths.map_filepath)
            .expect("Error loading map file");

        let rect = Rect::new(0., 0., world_size.x, world_size.y);

        // Create hashmap object_layer_name:Vec<rects>
        let mut object_rects = World::objects_to_rects(&map);

        // Extend Unmovable layer with edge rects
        object_rects
            .get_mut("Unmovable")
            .expect("Cannot get Unmovable value")
            .extend(World::edges_to_rects(&rect).iter());

        let world = World {
            texture_layers,
            rect,
            object_rects,
            map,
        };

        world
    }

    pub fn update(&mut self) {
        // Update movable world elements
    }

    fn edges_to_rects(rect: &Rect) -> Vec<Rect> {
        let mut rects: Vec<Rect> = vec![];
        rects.push(Rect {
            x: rect.x,
            y: rect.y,
            w: 1.,
            h: rect.h,
        });
        rects.push(Rect {
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h: 1.,
        });
        rects.push(Rect {
            x: rect.right(),
            y: rect.y,
            w: 1.,
            h: rect.h,
        });
        rects.push(Rect {
            x: rect.left(),
            y: rect.bottom(),
            w: rect.w,
            h: 1.,
        });
        rects
    }

    pub fn collision(&mut self, rect: &Rect, move_vector: Vec2) -> bool {
        let mut moved_rect = rect.clone();
        moved_rect.move_to(vec2(rect.x, rect.y) + move_vector);
        let mut collision = false;
        for mr in self
            .object_rects
            .get("Unmovable")
            .expect("Unmovable layer not found")
        {
            if moved_rect.overlaps(mr) {
                collision = true
            }
        }
        collision
    }

    fn objects_to_rects(map: &Map) -> HashMap<String, Vec<Rect>> {
        let mut rects: HashMap<String, Vec<Rect>> = HashMap::new();

        for layer in map.layers() {
            if let LayerType::Objects(_) = layer.layer_type() {
                let mut layer_objects: Vec<Rect> = Vec::new();
                let layer_name: String = layer.name.clone();

                let object_layer = layer
                    .as_object_layer()
                    .expect("Cannot parse layer as object layer");

                for object in object_layer.objects() {
                    match object.shape {
                        ObjectShape::Rect { width, height } => {
                            layer_objects.push(Rect::new(object.x, object.y, width, height));
                        }
                        _ => {}
                    }
                }

                rects.insert(layer_name, layer_objects);
            }
        }
        rects
    }
}
