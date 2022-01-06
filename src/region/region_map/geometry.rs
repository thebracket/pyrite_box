use super::material_bucket::Bucket;

pub const GEOMETRY_SIZE: f32 = 10.0;

pub(crate) const CUBE_NORMALS: [[f32; 3]; 6] = [
    [0.0, 0.0, -1.0],
    [0.0, 0.0, 1.0],
    [-1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, 1.0, 0.0],
];

pub(crate) const CUBE_TANGENTS: [[f32; 3]; 6] = [
    [-1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, -1.0],
    [1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
];

pub fn add_floor_geometry(
    bucket: &mut Bucket,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    let x0 = x * GEOMETRY_SIZE;
    let x1 = (x + w) * GEOMETRY_SIZE;
    let y0 = y * GEOMETRY_SIZE;
    let y1 = (y + h) * GEOMETRY_SIZE;
    let z0 = z * GEOMETRY_SIZE;

    #[rustfmt::skip]
    let cube_geometry = [
        [x0, y0, z0,],
        [x1, y0, z0,],
        [x1, y1, z0,],
        [x1, y1, z0,],
        [x0, y1, z0,],
        [x0, y0, z0,],
    ];
    bucket.vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
    ];
    bucket.normals.extend_from_slice(&NORMAL_GEOMETRY);

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
    ];
    bucket.tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 6] = [
        [0.0, 0.0],
        [tw, 0.0],
        [tw, th],
        [tw, th],
        [0.0, th],
        [0.0, 0.0],
    ];

    bucket.uv.extend_from_slice(&uv_base);

    //calculate_tangent(&cube_geometry, &uv_base);
    //println!("{:?}", CUBE_TANGENTS[1]);
}

pub fn add_cube_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32,
) {
    let x0 = x * GEOMETRY_SIZE;
    let x1 = (x + w) * GEOMETRY_SIZE;
    let y0 = y * GEOMETRY_SIZE;
    let y1 = (y + h) * GEOMETRY_SIZE;
    let z0 = z * GEOMETRY_SIZE;
    let z1 = (z + d) * GEOMETRY_SIZE;

    //println!("Cube at: {},{},{}", x0, y0, z0);

    #[rustfmt::skip]
    let cube_geometry = [
        [x0, y0, z0,],
        [x1, y1, z0,],
        [x1, y0, z0,],
        [x1, y1, z0,],
        [x0, y0, z0,],
        [x0, y1, z0,],

        [x0, y0, z1,],
        [x1, y0, z1,],
        [x1, y1, z1,],
        [x1, y1, z1,],
        [x0, y1, z1,],
        [x0, y0, z1,],

        [x0, y1, z1,],
        [x0, y1, z0,],
        [x0, y0, z0,],
        [x0, y0, z0,],
        [x0, y0, z1,],
        [x0, y1, z1,],

        [x1, y1, z1,],
        [x1, y0, z0,],
        [x1, y1, z0,],
        [x1, y0, z0,],
        [x1, y1, z1,],
        [x1, y0, z1,],

        [x0, y0, z0,],
        [x1, y0, z0,],
        [x1, y0, z1,],
        [x1, y0, z1,],
        [x0, y0, z1,],
        [x0, y0, z0,],

        [x1, y1, z1,],
        [x1, y1, z0,],
        [x0, y1, z0,],
        [x0, y1, z0,],
        [x0, y1, z1,],
        [x1, y1, z1,],
    ];
    vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 36] = [
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],

        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],

        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],

        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],

        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],

        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 36] = [
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],

        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],

        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],

        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],

        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],

        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 36] = [
        [0.0, 0.0],
        [tw, th],
        [tw, 0.0],
        [tw, th],
        [0.0, 0.0],
        [0.0, th],

        [0.0, 0.0],
        [tw, 0.0],
        [tw, th],
        [tw, th],
        [0.0, th],
        [0.0, 0.0],

        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],

        [tw, th],
        [0.0, 0.0],
        [tw, 0.0],
        [0.0, 0.0],
        [tw, th],
        [0.0, th],

        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],

        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],
    ];

    uv.extend_from_slice(&uv_base);
}

pub fn add_ceiling_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    let x0 = x * GEOMETRY_SIZE;
    let x1 = (x + w) * GEOMETRY_SIZE;
    let y0 = y * GEOMETRY_SIZE;
    let y1 = (y + h) * GEOMETRY_SIZE;
    //let z0 = z * GEOMETRY_SIZE;
    let z1 = (z + 1.0) * GEOMETRY_SIZE;

    #[rustfmt::skip]
    let cube_geometry = [
        [x0, y0, z1,],
        [x1, y1, z1,],
        [x1, y0, z1,],
        [x1, y1, z1,],
        [x0, y0, z1,],
        [x0, y1, z1,],
    ];
    vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 6] = [
        [0.0, 0.0],
        [tw, 0.0],
        [tw, th],
        [tw, th],
        [0.0, th],
        [0.0, 0.0],
    ];

    uv.extend_from_slice(&uv_base);
}

pub fn add_south_facing_wall_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    let x0 = x * GEOMETRY_SIZE;
    let x1 = (x + w) * GEOMETRY_SIZE;
    let y0 = y * GEOMETRY_SIZE;
    //let y1 = (y + h) * GEOMETRY_SIZE;
    let z0 = z * GEOMETRY_SIZE;
    let z1 = (z + 1.0) * GEOMETRY_SIZE;

    #[rustfmt::skip]
    let cube_geometry = [
        [x1, y0, z1,],
        [x1, y0, z0,],
        [x0, y0, z0,],
        [x0, y0, z0,],
        [x0, y0, z1,],
        [x1, y0, z1,],
    ];
    vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 6] = [
        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],
    ];

    uv.extend_from_slice(&uv_base);
}

pub fn add_north_facing_wall_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    let x0 = x * GEOMETRY_SIZE;
    let x1 = (x + w) * GEOMETRY_SIZE;
    //let y0 = y * GEOMETRY_SIZE;
    let y1 = (y + h) * GEOMETRY_SIZE;
    let z0 = z * GEOMETRY_SIZE;
    let z1 = (z + 1.0) * GEOMETRY_SIZE;

    #[rustfmt::skip]
    let cube_geometry = [
        [x0, y1, z0,],
        [x1, y1, z0,],
        [x1, y1, z1,],
        [x1, y1, z1,],
        [x0, y1, z1,],
        [x0, y1, z0,],
    ];
    vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 6] = [
        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],
    ];

    uv.extend_from_slice(&uv_base);
}

pub fn add_east_facing_wall_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    //let x0 = x * GEOMETRY_SIZE;
    let x1 = (x + w) * GEOMETRY_SIZE;
    let y0 = y * GEOMETRY_SIZE;
    let y1 = (y + h) * GEOMETRY_SIZE;
    let z0 = z * GEOMETRY_SIZE;
    let z1 = (z + 1.0) * GEOMETRY_SIZE;

    #[rustfmt::skip]
    let cube_geometry = [
        [x1, y1, z1,],
        [x1, y1, z0,],
        [x1, y0, z0,],
        [x1, y0, z0,],
        [x1, y0, z1,],
        [x1, y1, z1,],
    ];
    vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 6] = [
        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],
    ];

    uv.extend_from_slice(&uv_base);
}

pub fn add_west_facing_wall_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    let x0 = x * GEOMETRY_SIZE;
    //let x1 = (x + w) * GEOMETRY_SIZE;
    let y0 = y * GEOMETRY_SIZE;
    let y1 = (y + h) * GEOMETRY_SIZE;
    let z0 = z * GEOMETRY_SIZE;
    let z1 = (z + 1.0) * GEOMETRY_SIZE;

    #[rustfmt::skip]
    let cube_geometry = [
        [x0, y1, z1,],
        [x0, y0, z0,],
        [x0, y1, z0,],
        [x0, y0, z0,],
        [x0, y1, z1,],
        [x0, y0, z1,],
    ];
    vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 6] = [
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 6] = [
        [tw, th],
        [0.0, 0.0],
        [tw, 0.0],
        [0.0, 0.0],
        [tw, th],
        [0.0, th],
    ];

    uv.extend_from_slice(&uv_base);
}

/*fn calculate_tangent(pos: &[[f32;3]], uv: &[[f32;2]]) {
    use bevy::math::{Vec3, Vec2};
    let pos1 = Vec3::from(pos[0]);
    let pos2 = Vec3::from(pos[1]);
    let pos3 = Vec3::from(pos[2]);
    let pos4 = Vec3::from(pos[4]);
    let uv1 = Vec2::from(uv[0]);
    let uv2 = Vec2::from(uv[1]);
    let uv3 = Vec2::from(uv[2]);
    let uv4 = Vec2::from(uv[4]);


    let edge1 = pos2 - pos1;
    let edge2 = pos3 - pos1;
    let delta_uv1 = uv2 - uv1;
    let delta_uv2 = uv3 - uv1;

    let f = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y);
    let tangent1 = Vec3::from([
        f * (delta_uv2.y * edge1.x - delta_uv1.y * edge2.x),
        f * (delta_uv2.y * edge1.y - delta_uv1.y * edge2.y),
        f * (delta_uv2.y * edge1.z - delta_uv1.y * edge2.z),
    ]).normalize();

    println!("{:?}", tangent1);
}*/

pub fn add_south_facing_opening_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    add_south_facing_wall_geometry(vertices, normals, uv, tangents, x, y, z, w / 4.0, h);
    add_south_facing_wall_geometry(vertices, normals, uv, tangents, x + 0.75, y, z, w / 4.0, h);
}

pub fn add_north_facing_opening_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    add_north_facing_wall_geometry(vertices, normals, uv, tangents, x, y, z, w / 4.0, h);
    add_north_facing_wall_geometry(vertices, normals, uv, tangents, x + 0.75, y, z, w / 4.0, h);
}

pub fn add_east_facing_opening_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    add_east_facing_wall_geometry(vertices, normals, uv, tangents, x, y, z, w, h / 4.0);
    add_east_facing_wall_geometry(vertices, normals, uv, tangents, x, y + 0.75, z, w, h / 4.0);
}

pub fn add_west_facing_opening_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
) {
    add_west_facing_wall_geometry(vertices, normals, uv, tangents, x, y, z, w, h / 4.0);
    add_west_facing_wall_geometry(vertices, normals, uv, tangents, x, y + 0.75, z, w, h / 4.0);
}
