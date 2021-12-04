pub mod geometry;
use serde::{Deserialize, Serialize};
mod material_bucket;
use self::material_bucket::{FeatureType, MaterialBucket};
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
pub mod map_editor;
use crate::module::Direction;

#[derive(Clone, Serialize, Deserialize)]
pub struct RegionMap {
    pub name: String,
    pub size: (u32, u32),
    pub tiles: Vec<RegionTile>,
    pub starting_location: (u32, u32, Direction),
    pub needs_rebuild: bool,
    pub map_start_event: String,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct RegionTile {
    pub tile_type: RegionTileType,
    pub has_ceiling: bool,
    pub boundaries: [(RegionBoundaryType, u32); 4],
    pub floor_material: u32,
    pub ceiling_material: u32,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionTileType {
    Empty,
    Floor,
    Solid,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RegionBoundaryType {
    None,
    Wall,
    Opening,
}

impl RegionMap {
    pub fn default() -> Self {
        const SIZE: (u32, u32) = (16, 16);
        let mut map = RegionMap {
            name: String::from("Test Map"),
            size: (SIZE.0, SIZE.1),
            tiles: vec![
                RegionTile {
                    has_ceiling: false,
                    tile_type: RegionTileType::Floor,
                    boundaries: [
                        (RegionBoundaryType::None, 1),
                        (RegionBoundaryType::None, 1),
                        (RegionBoundaryType::None, 1),
                        (RegionBoundaryType::None, 1),
                    ],
                    floor_material: 0,
                    ceiling_material: 1,
                };
                (SIZE.0 as usize) * (SIZE.1 as usize)
            ],
            starting_location: (SIZE.0 / 2, SIZE.1 / 2, Direction::North),
            needs_rebuild: false,
            map_start_event: String::new(),
        };

        for x in 0..SIZE.0 {
            map.tiles[((0 * SIZE.0) + x) as usize].boundaries
                [Direction::North.to_exit_index() as usize]
                .0 = RegionBoundaryType::Wall;
            map.tiles[(((SIZE.1 - 1) * SIZE.0) + x) as usize].boundaries
                [Direction::South.to_exit_index()]
            .0 = RegionBoundaryType::Wall;
        }
        for y in 0..SIZE.1 {
            map.tiles[((y * SIZE.0) + 0) as usize].boundaries[Direction::West.to_exit_index()].0 =
                RegionBoundaryType::Wall;
            map.tiles[((y * SIZE.0) + (SIZE.0 - 1)) as usize].boundaries
                [Direction::East.to_exit_index()]
            .0 = RegionBoundaryType::Wall;
        }

        map
    }

    pub fn tile_location(&self, x: f32, y: f32) -> (f32, f32) {
        (0.0 - x, y)
    }

    pub fn create_geometry(&self, meshes: &mut Assets<Mesh>) -> Vec<(u32, Handle<Mesh>)> {
        let mut bucket = MaterialBucket::new();
        let mut result = Vec::new();

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let (sx, sy) = self.tile_location(x as f32, y as f32);
                let tile_idx = ((self.size.0 * y) + x) as usize;

                if self.tiles[tile_idx].has_ceiling {
                    bucket.add_feature(
                        FeatureType::Ceiling,
                        self.tiles[tile_idx].ceiling_material,
                        sx,
                        sy,
                    );
                }

                match self.tiles[tile_idx].tile_type {
                    RegionTileType::Floor => {
                        bucket.add_feature(
                            FeatureType::Floor,
                            self.tiles[tile_idx].floor_material,
                            sx,
                            sy,
                        );
                    }
                    RegionTileType::Solid => {
                        bucket.add_feature(
                            FeatureType::Cube,
                            self.tiles[tile_idx].floor_material,
                            sx,
                            sy,
                        );
                    }
                    RegionTileType::Empty => {}
                }

                if self.tiles[tile_idx].boundaries[Direction::North.to_exit_index()].0
                    == RegionBoundaryType::Wall
                {
                    bucket.add_feature(
                        FeatureType::Wall(Direction::South),
                        self.tiles[tile_idx].boundaries[Direction::North.to_exit_index()].1,
                        sx,
                        sy,
                    );
                }
                if self.tiles[tile_idx].boundaries[Direction::South.to_exit_index()].0
                    == RegionBoundaryType::Wall
                {
                    bucket.add_feature(
                        FeatureType::Wall(Direction::North),
                        self.tiles[tile_idx].boundaries[Direction::South.to_exit_index()].1,
                        sx,
                        sy,
                    );
                }
                if self.tiles[tile_idx].boundaries[Direction::East.to_exit_index()].0
                    == RegionBoundaryType::Wall
                {
                    bucket.add_feature(
                        FeatureType::Wall(Direction::West),
                        self.tiles[tile_idx].boundaries[Direction::East.to_exit_index()].1,
                        sx,
                        sy,
                    );
                }
                if self.tiles[tile_idx].boundaries[Direction::West.to_exit_index()].0
                    == RegionBoundaryType::Wall
                {
                    bucket.add_feature(
                        FeatureType::Wall(Direction::East),
                        self.tiles[tile_idx].boundaries[Direction::West.to_exit_index()].1,
                        sx,
                        sy,
                    );
                }
            }
        }

        for (material_id, bucket) in bucket.materials.drain() {
            let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
            mesh.set_attribute(
                Mesh::ATTRIBUTE_POSITION,
                VertexAttributeValues::Float3(bucket.vertices),
            );
            mesh.set_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                VertexAttributeValues::Float3(bucket.normals),
            );
            mesh.set_attribute(
                Mesh::ATTRIBUTE_UV_0,
                VertexAttributeValues::Float2(bucket.uv),
            );
            mesh.set_attribute(
                Mesh::ATTRIBUTE_TANGENT,
                VertexAttributeValues::Float3(bucket.tangents),
            );

            result.push((material_id, meshes.add(mesh)));
        }

        result
    }
}
