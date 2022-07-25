mod biome;
mod voxel_data;

use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use self::voxel_data::*;

pub struct WorldGen;

struct Voxel {
    position: Vec3,
    value: bool,
}

impl Plugin for WorldGen {
    fn build(&self, app: &mut App) {
        app.add_startup_system(gen_chunk);
    }
}

fn gen_chunk(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut triangles: Vec<u32> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let mut voxelmap: Vec<Voxel> = Vec::new();

    let mut vertex_index = 0;

    populate_voxelmap(&mut voxelmap);

    create_mesh_data(
        &mut voxelmap,
        &mut vertices,
        &mut triangles,
        &mut uvs,
        &mut vertex_index
    );

    create_mesh(
        commands,
        meshes,
        materials,
        vertices,
        triangles,
        uvs
    );
}

fn populate_voxelmap(voxelmap: &mut Vec<Voxel>) {
    for y in 0..CHUNK_HEIGHT {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                let mut val: bool = true;
                if rand::random() {
                    val = false;
                }

                voxelmap.push(Voxel { position: Vec3::new(x as f32, y as f32, z as f32), value: val });
            }
        }
    }
}

fn create_mesh_data(
    mut voxelmap: &mut Vec<Voxel>,
    mut vertices: &mut Vec<Vec3>,
    mut triangles: &mut Vec<u32>,
    mut uvs: &mut Vec<[f32; 2]>,
    mut vertex_index: &mut i32,
) {
    for y in 0..CHUNK_HEIGHT {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                add_voxel_data_to_chunk(
                    Vec3::new(x as f32, y as f32, z as f32),
                    &mut voxelmap,
                    &mut vertices,
                    &mut triangles,
                    &mut uvs,
                    &mut vertex_index
                );
            }
        }
    }
}

fn check_voxel(
    pos: Vec3,
    voxelmap: &mut Vec<Voxel>,
) -> bool {
    let x: i32 = pos.x.round() as i32;
    let y: i32 = pos.y.round() as i32;
    let z: i32 = pos.z.round() as i32;

    if x < 0 || x > CHUNK_WIDTH - 1 || y < 0 || y > CHUNK_HEIGHT - 1 || z < 0 || z > CHUNK_WIDTH - 1 {
        return false;
    }

    for v in voxelmap {
        if v.position.x.round() as i32 == x && v.position.y.round() as i32 == y && v.position.z.round() as i32 == z {
            return v.value;
        }
    }

    return false;
}

fn add_voxel_data_to_chunk(
    pos: Vec3,
    voxelmap: &mut Vec<Voxel>,
    vertices: &mut Vec<Vec3>,
    triangles: &mut Vec<u32>,
    uvs: &mut Vec<[f32; 2]>,
    vertex_index: &mut i32,
) {
    for p in 0..6 {
        let mut v = FACE_CHECKS[p];
        if !check_voxel(pos + Vec3::new(v[0], v[1], v[2]), voxelmap) {
            // Vertices
            v = VERTICES[TRIANGLES[p][0]];
            vertices.push(pos + Vec3::new(v[0], v[1], v[2]));

            v = VERTICES[TRIANGLES[p][1]];
            vertices.push(pos + Vec3::new(v[0], v[1], v[2]));

            v = VERTICES[TRIANGLES[p][2]];
            vertices.push(pos + Vec3::new(v[0], v[1], v[2]));

            v = VERTICES[TRIANGLES[p][3]];
            vertices.push(pos + Vec3::new(v[0], v[1], v[2]));

            // UVs
            uvs.push(UVS[0]);
            uvs.push(UVS[1]);
            uvs.push(UVS[2]);
            uvs.push(UVS[3]);
        }
    }

    // Triangles
    triangles.push(*vertex_index as u32);
    triangles.push((*vertex_index + 1) as u32);
    triangles.push((*vertex_index + 2) as u32);
    triangles.push((*vertex_index + 2) as u32);
    triangles.push((*vertex_index + 1) as u32);
    triangles.push((*vertex_index + 3) as u32);
    *vertex_index = *vertex_index + 4;
}

fn create_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    vertices: Vec<Vec3>,
    triangles: Vec<u32>,
    uvs: Vec<[f32; 2]>,
) {
    let mut positions = Vec::new();


    for p in vertices {
        let pos: [f32;3] = [p.x, p.y, p.z];
        positions.push(pos);
    }

    let indices = mesh::Indices::U32(triangles);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("55bf0a").unwrap(),
            metallic: 0.0,
            perceptual_roughness: 1.0,
            ..Default::default()
        }),
        ..Default::default()
    });
}