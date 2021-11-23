use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use super::geometry::{
    add_ceiling_geometry, add_cube_geometry, add_east_facing_wall_geometry, add_floor_geometry,
    add_north_facing_wall_geometry, add_south_facing_wall_geometry, add_west_facing_wall_geometry,
    GEOMETRY_SIZE,
};

pub struct RegionMap {
    pub name: String,
    pub size: (u32, u32),
    pub tiles: Vec<RegionTile>,
    pub starting_location: (f32, f32, f32),
}

pub const NORTH: usize = 0;
pub const SOUTH: usize = 1;
pub const EAST: usize = 2;
pub const WEST: usize = 3;

#[derive(Clone, Copy)]
pub struct RegionTile {
    pub tile_type: RegionTileType,
    pub has_ceiling: bool,
    pub boundaries: [RegionBoundaryType; 4],
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RegionTileType {
    EMPTY,
    FLOOR,
    SOLID,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RegionBoundaryType {
    NONE,
    WALL,
}

impl RegionMap {
    pub fn default() -> Self {
        const SIZE: (u32, u32) = (32, 32);
        let mut map = RegionMap {
            name: String::from("Test Map"),
            size: (SIZE.0, SIZE.1),
            tiles: vec![
                RegionTile {
                    has_ceiling: false,
                    tile_type: RegionTileType::FLOOR,
                    boundaries: [
                        RegionBoundaryType::NONE,
                        RegionBoundaryType::NONE,
                        RegionBoundaryType::NONE,
                        RegionBoundaryType::NONE,
                    ]
                };
                (SIZE.0 as usize) * (SIZE.1 as usize)
            ],
            starting_location: (
                ((SIZE.0 / 2) as f32 * GEOMETRY_SIZE) + GEOMETRY_SIZE / 2.0,
                ((SIZE.1 / 2) as f32 * GEOMETRY_SIZE) + GEOMETRY_SIZE / 2.0,
                GEOMETRY_SIZE * 0.5,
            ),
        };

        for x in 0..SIZE.0 {
            map.tiles[((0 * SIZE.0) + x) as usize].boundaries[NORTH] = RegionBoundaryType::WALL;
            map.tiles[(((SIZE.1-1) * SIZE.0) + x) as usize].boundaries[SOUTH] = RegionBoundaryType::WALL;
        }
        for y in 0..SIZE.1 {
            map.tiles[((y * SIZE.0) + 0) as usize].boundaries[EAST] = RegionBoundaryType::WALL;
            map.tiles[((y * SIZE.0) + (SIZE.0-1)) as usize].boundaries[WEST] = RegionBoundaryType::WALL;
        }

        map
    }

    fn tile_location(&self, x: f32, y: f32) -> (f32, f32) {
        (x, y)
    }

    pub fn create_geometry(&self, meshes: &mut Assets<Mesh>) -> Vec<Handle<Mesh>> {
        let mut result = Vec::new();
        let mut vertices = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut uv = Vec::<[f32; 2]>::new();
        let mut tangents = Vec::<[f32; 3]>::new();

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let (sx, sy) = self.tile_location(x as f32, y as f32);
                let tile_idx = ((self.size.0 * y) + x) as usize;

                if self.tiles[tile_idx].has_ceiling {
                    add_ceiling_geometry(
                        &mut vertices,
                        &mut normals,
                        &mut uv,
                        &mut tangents,
                        sx,
                        sy,
                        0.0,
                        1.0,
                        1.0,
                    );
                }

                match self.tiles[tile_idx].tile_type {
                    RegionTileType::FLOOR => {
                        add_floor_geometry(
                            &mut vertices,
                            &mut normals,
                            &mut uv,
                            &mut tangents,
                            sx,
                            sy,
                            0.0,
                            1.0,
                            1.0,
                        );
                    }
                    RegionTileType::SOLID => {
                        add_cube_geometry(
                            &mut vertices,
                            &mut normals,
                            &mut uv,
                            &mut tangents,
                            sx,
                            sy,
                            0.0,
                            1.0,
                            1.0,
                            1.0,
                        );
                    }
                    RegionTileType::EMPTY => {}
                }

                if self.tiles[tile_idx].boundaries[NORTH] == RegionBoundaryType::WALL {
                    add_south_facing_wall_geometry(
                        &mut vertices,
                        &mut normals,
                        &mut uv,
                        &mut tangents,
                        sx,
                        sy,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
                if self.tiles[tile_idx].boundaries[SOUTH] == RegionBoundaryType::WALL {
                    add_north_facing_wall_geometry(
                        &mut vertices,
                        &mut normals,
                        &mut uv,
                        &mut tangents,
                        sx,
                        sy,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
                if self.tiles[tile_idx].boundaries[WEST] == RegionBoundaryType::WALL {
                    add_east_facing_wall_geometry(
                        &mut vertices,
                        &mut normals,
                        &mut uv,
                        &mut tangents,
                        sx,
                        sy,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
                if self.tiles[tile_idx].boundaries[EAST] == RegionBoundaryType::WALL {
                    add_west_facing_wall_geometry(
                        &mut vertices,
                        &mut normals,
                        &mut uv,
                        &mut tangents,
                        sx,
                        sy,
                        0.0,
                        1.0,
                        1.0,
                    );
                }
            }
        }

        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        mesh.set_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float3(vertices),
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float3(normals),
        );
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float2(uv));
        mesh.set_attribute(
            Mesh::ATTRIBUTE_TANGENT,
            VertexAttributeValues::Float3(tangents),
        );

        result.push(meshes.add(mesh));

        result
    }
}
