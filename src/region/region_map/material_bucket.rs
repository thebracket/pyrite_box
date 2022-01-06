use crate::module::Direction;
use std::collections::HashMap;

use super::geometry::*;

pub enum FeatureType {
    Floor,
    Ceiling,
    Wall(Direction),
    Cube,
    Opening(Direction),
}

pub struct MaterialBucket {
    pub materials: HashMap<u32, Bucket>,
}

impl MaterialBucket {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
        }
    }

    fn add_feature_to_bucket(bucket: &mut Bucket, feature: FeatureType, x: f32, y: f32) {
        match feature {
            FeatureType::Floor => {
                add_floor_geometry(
                    bucket,
                    x,
                    y,
                    0.0,
                    1.0,
                    1.0,
                );
            }
            FeatureType::Ceiling => {
                add_ceiling_geometry(
                    &mut bucket.vertices,
                    &mut bucket.normals,
                    &mut bucket.uv,
                    &mut bucket.tangents,
                    x,
                    y,
                    0.0,
                    1.0,
                    1.0,
                );
            }
            FeatureType::Wall(dir) => match dir {
                Direction::East => {
                    add_east_facing_wall_geometry(
                        &mut bucket.vertices,
                        &mut bucket.normals,
                        &mut bucket.uv,
                        &mut bucket.tangents,
                        x,
                        y,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
                Direction::North => {
                    add_north_facing_wall_geometry(
                        &mut bucket.vertices,
                        &mut bucket.normals,
                        &mut bucket.uv,
                        &mut bucket.tangents,
                        x,
                        y,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
                Direction::South => {
                    add_south_facing_wall_geometry(
                        &mut bucket.vertices,
                        &mut bucket.normals,
                        &mut bucket.uv,
                        &mut bucket.tangents,
                        x,
                        y,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
                Direction::West => {
                    add_west_facing_wall_geometry(
                        &mut bucket.vertices,
                        &mut bucket.normals,
                        &mut bucket.uv,
                        &mut bucket.tangents,
                        x,
                        y,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
            },
            FeatureType::Opening(dir) => match dir {
                Direction::East => {
                    add_east_facing_opening_geometry(
                        &mut bucket.vertices,
                        &mut bucket.normals,
                        &mut bucket.uv,
                        &mut bucket.tangents,
                        x,
                        y,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
                Direction::North => {
                    add_north_facing_opening_geometry(
                        &mut bucket.vertices,
                        &mut bucket.normals,
                        &mut bucket.uv,
                        &mut bucket.tangents,
                        x,
                        y,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
                Direction::South => {
                    add_south_facing_opening_geometry(
                        &mut bucket.vertices,
                        &mut bucket.normals,
                        &mut bucket.uv,
                        &mut bucket.tangents,
                        x,
                        y,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
                Direction::West => {
                    add_west_facing_opening_geometry(
                        &mut bucket.vertices,
                        &mut bucket.normals,
                        &mut bucket.uv,
                        &mut bucket.tangents,
                        x,
                        y,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
            },
            FeatureType::Cube => {
                add_cube_geometry(
                    &mut bucket.vertices,
                    &mut bucket.normals,
                    &mut bucket.uv,
                    &mut bucket.tangents,
                    x,
                    y,
                    0.0,
                    1.0,
                    1.0,
                    1.0,
                );
            }
        }
    }

    pub fn add_feature(&mut self, feature: FeatureType, material_id: u32, x: f32, y: f32) {
        if let Some(bucket) = self.materials.get_mut(&material_id) {
            MaterialBucket::add_feature_to_bucket(bucket, feature, x, y);
        } else {
            let mut bucket = Bucket::new();
            MaterialBucket::add_feature_to_bucket(&mut bucket, feature, x, y);
            self.materials.insert(material_id, bucket);
        }
    }
}

pub struct Bucket {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub tangents: Vec<[f32; 3]>,
}

impl Bucket {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            uv: Vec::new(),
            tangents: Vec::new(),
        }
    }
}
