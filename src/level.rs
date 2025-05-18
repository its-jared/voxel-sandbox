use std::sync::Arc;
use bevy_voxel_world::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*};
use noise::{HybridMulti, NoiseFn, Perlin};

use crate::voxels;

#[derive(Resource, Clone, Default)]
pub struct MainLevel;

impl VoxelWorldConfig for MainLevel {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        5
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        Box::new(move |_chunk_pos| build_world())
    }

    fn texture_index_mapper(
        &self,
    ) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|mat| match mat {
            voxels::MOSS => [0, 2, 1],
            voxels::STONE  => [1, 1, 1],
            voxels::WATER | _ => [3, 3, 3], 
        })
    }

    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(("voxels.png".into(), 4))
    }
}

fn build_world() -> Box<dyn FnMut(IVec3) -> WorldVoxel + Send + Sync> {
    let mut n1 = HybridMulti::<Perlin>::new(1234);
    n1.octaves = 5;
    n1.frequency = 0.1;
    n1.lacunarity = 2.8;
    n1.persistence = 0.4;

    let mut n2 = HybridMulti::<Perlin>::new(1234);
    n2.octaves = 5;
    n2.frequency = 0.5;
    n2.lacunarity = 2.8;
    n2.persistence = 0.4;

    let mut cache = HashMap::<(i32, i32), f64>::new();

    Box::new(move |pos: IVec3| {
        let [x, y, z] = pos.as_dvec3().to_array();

        let is_ground = y < match cache.get(&(pos.x, pos.z)) {
            Some(sample) => *sample,
            None => {
                let sample1 = n2.get([x / 1000.0, z / 1000.0]);
                let sample = n1.get([x * sample1, z * sample1]) * 10.0;
                cache.insert((pos.x, pos.z), sample);
                sample
            }
        };

        if is_ground {
            WorldVoxel::Solid(voxels::MOSS)
        }
        else {
            WorldVoxel::Air
        }
    })
}