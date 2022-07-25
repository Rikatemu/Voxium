pub const CHUNK_WIDTH: i32 = 16;
pub const CHUNK_HEIGHT: i32 = 16;

pub const VERTICES: [[f32; 3]; 8] = [
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0],
    [0.0, 1.0, 1.0],
];

pub const UVS: [[f32; 2]; 8] = [
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 0.0],
    [1.0, 1.0],
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 0.0],
    [1.0, 1.0],
];

pub const TRIANGLES: [[usize; 4]; 6] = [
    [0, 3, 1, 2],
    [5, 6, 4, 7],
    [3, 7, 2, 6],
    [1, 5, 0, 4],
    [4, 7, 0, 3],
    [1, 2, 5, 6],
];

pub const FACE_CHECKS: [[f32; 3]; 6] = [
    [0.0, 0.0, -1.0],
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0],
    [0.0, -1.0, 0.0],
    [-1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
];