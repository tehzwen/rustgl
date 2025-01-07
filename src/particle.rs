
// basically like an object, only need one set of buffers, but then more data for each individual particle? (lifespan, world pos etc?)

use nalgebra::Vector3;

use crate::buffers::RenderBuffers;

pub struct Particle {
    pub position: Vector3<f32>,

}

pub struct ParticleGenerator {
    pub buffers: RenderBuffers,
    pub num_particles: i32
}

impl ParticleGenerator {
    pub fn init(self) {
        // we need a buffer for all of the positions



    }
}