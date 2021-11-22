use bevy::prelude::*;

use crate::{Particle, ParticleHandle, TrailMap};

pub fn render(
    mut textures: ResMut<Assets<Texture>>,
    tex_handle: Res<ParticleHandle>,
    query: Query<&Particle>,
    trail_map: Res<TrailMap>,
) {
    if let Some(texture) = textures.get_mut(&tex_handle.handle) {
        for pixel in texture.data.chunks_mut(4) {
            // r g b (a)
            pixel[3] = 0;
        }

        for particle in query.iter() {
            let index = particle.y * texture.size.width as i32 + particle.x;
            if let Some(pixel) = texture.data.chunks_mut(4).nth(index as usize) {
                pixel[3] = 255;
            }
        }

        for (cell, pixel) in trail_map.trail.iter().zip(texture.data.chunks_mut(4)) {
            pixel[3] = (255.0 * cell / 100.0) as u8;
        }
    }
}
