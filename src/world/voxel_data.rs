pub const CHUNK_WIDTH: i32 = 32;
pub const CHUNK_HEIGHT: i32 = 8;

pub const VERTICES: [[f32; 3]; 8] = [
    [0.0, 0.0, 0.0], // 0
    [1.0, 0.0, 0.0], // 1
    [1.0, 1.0, 0.0], // 2
    [0.0, 1.0, 0.0], // 3
    [0.0, 0.0, 1.0], // 4
    [1.0, 0.0, 1.0], // 5
    [1.0, 1.0, 1.0], // 6
    [0.0, 1.0, 1.0], // 7
];

pub const UVS: [[f32; 2]; 4] = [
    [0.0, 0.0], // 0
    [0.0, 1.0], // 1
    [1.0, 0.0], // 2
    [1.0, 1.0], // 3
];

pub const TRIANGLES: [[usize; 4]; 6] = [
    [0, 3, 1, 2], // 0 front
    [5, 6, 4, 7], // 1 back
    [3, 7, 2, 6], // 2 top
    [1, 5, 0, 4], // 3 bottom
    [4, 7, 0, 3], // 4 left
    [1, 2, 5, 6], // 5 right
];

pub const FACE_CHECKS: [[f32; 3]; 6] = [
    [0.0, 0.0, -1.0], // 0
    [0.0, 0.0, 1.0], // 1
    [0.0, 1.0, 0.0], // 2
    [0.0, -1.0, 0.0], // 3
    [-1.0, 0.0, 0.0], // 4
    [1.0, 0.0, 0.0], // 5
];