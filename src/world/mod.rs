mod biome;
mod voxel_data;

use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use self::voxel_data::*;

pub struct WorldGen;

/*
 * Voxel data
 * postion: position of the voxel in chunk
 * solid: whether the voxel has collision or not (if false render surrounding voxel faces) -> TODO: move this to a block database and don't save it into the voxel
 * block type: id of the block type
 */
struct Voxel {
    position: Vec3,
    solid: bool,
    //block_type: u32,
}

impl Plugin for WorldGen {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_chunk);
    }
}

fn generate_chunk(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    // Chunk data
    let mut voxelmap: Vec<Voxel> = Vec::new();
    // Chunk mesh data
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut triangles: Vec<u32> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut vertex_index: i32 = 0;

    // Generate chunk block data
    generate_voxelmap(&mut voxelmap);

    // Generate chunk mesh data for rendering
    generate_mesh_data(&mut voxelmap, &mut vertices, &mut triangles, &mut uvs, &mut vertex_index);

    // Finally render chunk mesh
    render_chunk_mesh(
        commands,
        meshes,
        materials,
        vertices,
        triangles,
        uvs
    );
}

fn generate_voxelmap(
    voxelmap: &mut Vec<Voxel>,
) {
    for y in 0..CHUNK_HEIGHT {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                // TODO: add procedural algorithms here
                let mut sl: bool = true;

                // Randomly place blocks
                if rand::random() {
                    sl = false;
                }

                let voxel = Voxel {
                    position: Vec3::new(x as f32, y as f32, z as f32),
                    solid: sl,
                    //block_type: 0,
                };

                voxelmap.push(voxel);
            }
        }
    }
}

fn generate_mesh_data(
    voxelmap: &mut Vec<Voxel>,
    vertices: &mut Vec<Vec3>,
    triangles: &mut Vec<u32>,
    uvs: &mut Vec<[f32; 2]>,
    vertex_index: &mut i32,
) {
    for y in 0..CHUNK_HEIGHT {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                // Generate face data for each block
                generate_face_data(
                    Vec3::new(x as f32, y as f32, z as f32),
                    voxelmap, 
                    vertices, 
                    triangles,
                    uvs,
                    vertex_index,
                );
            }
        }
    }
}

fn generate_face_data(
    position: Vec3,
    voxelmap: &mut Vec<Voxel>,
    vertices: &mut Vec<Vec3>,
    triangles: &mut Vec<u32>,
    uvs: &mut Vec<[f32; 2]>,
    vertex_index: &mut i32,
) {
    if is_block_inside_chunk(position) && is_block_solid(voxelmap, position) {
        // For each of block's 6 faces
        for f in 0..6 {
            // Get neighbouring block's position
            let face_check: [f32; 3] = FACE_CHECKS[f];
            let neighbour_position: Vec3 = position + Vec3::new(face_check[0], face_check[1], face_check[2]);

            // Check neighbouring block, if it's solid, don't render this face
            if !is_block_solid(voxelmap, neighbour_position) {

                // Vertices
                let mut v: [f32; 3] = VERTICES[TRIANGLES[f][0]];
                vertices.push(position + Vec3::new(v[0], v[1], v[2]));

                v = VERTICES[TRIANGLES[f][1]];
                vertices.push(position + Vec3::new(v[0], v[1], v[2]));

                v = VERTICES[TRIANGLES[f][2]];
                vertices.push(position + Vec3::new(v[0], v[1], v[2]));

                v = VERTICES[TRIANGLES[f][3]];
                vertices.push(position + Vec3::new(v[0], v[1], v[2]));

                // UVs
                uvs.push(UVS[0]);
                uvs.push(UVS[1]);
                uvs.push(UVS[2]);
                uvs.push(UVS[3]);

                // Triangles
                triangles.push(*vertex_index as u32);
                triangles.push((*vertex_index + 1) as u32);
                triangles.push((*vertex_index + 2) as u32);
                triangles.push((*vertex_index + 2) as u32);
                triangles.push((*vertex_index + 1) as u32);
                triangles.push((*vertex_index + 3) as u32);

                *vertex_index = *vertex_index + 4;
            }
        }
    }
}

fn is_block_inside_chunk(
    position: Vec3,
) -> bool {
    let x: i32 = position.x.round() as i32;
    let y: i32 = position.y.round() as i32;
    let z: i32 = position.z.round() as i32;

    if x < 0 || x > CHUNK_WIDTH || y < 0 || y > CHUNK_HEIGHT || z < 0 || z > CHUNK_WIDTH {
        return false;
    }

    return true;
}

fn is_block_solid(
    voxelmap: &mut Vec<Voxel>,
    position: Vec3,
) -> bool {
    let x: f32 = position.x.round();
    let y: f32 = position.y.round();
    let z: f32 = position.z.round();

    // Iterate through voxelmap to find block on specific position and return its solid value
    // TODO: this uses a lot of processing resources, maybe we can optimize it
    for b in voxelmap {
        if b.position.x == x && b.position.y == y && b.position.z == z {
            return b.solid;
        }
    }

    return false;
}

fn render_chunk_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    vertices: Vec<Vec3>,
    triangles: Vec<u32>,
    uvs: Vec<[f32; 2]>,
) {
    let mut positions: Vec<[f32; 3]> = Vec::new();

    for p in vertices {
        let pos: [f32;3] = [p.x, p.y, p.z];
        positions.push(pos);
    }

    let mut mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(mesh::Indices::U32(triangles)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            metallic: 0.0,
            perceptual_roughness: 0.5,
            ..Default::default()
        }),
        ..default()
    });
}