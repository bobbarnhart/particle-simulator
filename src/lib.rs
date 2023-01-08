use std::collections::HashMap;

enum Side {
    BLUE,
    RED,
    GREEN,
    YELLOW,
}

struct Particle {
    name: String,
    side: Side,
    radius: f32,
    position: [f32; 3],
    velocity: [f32; 3]
}
struct ParticleSimulator {
    step_size: f32,
    time_seconds: f32,
    particles: HashMap<u32, Particle>,
    particle_index: u32
}


pub fn create_position(x: f32, y: f32, z: f32) -> [f32; 3] {
    // Create new particle location
    return [x, y, z];
}

fn add_particle(mut simulator: ParticleSimulator, particle: Particle) -> ParticleSimulator {
    // add a new particle to the simulator
    simulator.particle_index +=1;
    simulator.particles.insert(simulator.particle_index, particle);
    return simulator;
}

fn initialize_simulation(step_size: f32) -> ParticleSimulator {
    let time_seconds = 0.0;
    let index: u32 = 0;
    let particles: HashMap<u32, Particle> = HashMap::new();
    let simulator = ParticleSimulator {
        step_size: step_size,
        time_seconds: time_seconds,
        particles: particles,
        particle_index: index
    };

    return simulator;
}

fn advance_simulation(simulator:&ParticleSimulator, steps: u32) -> &ParticleSimulator {

    let mut remaining_steps: u32 = steps;

    while remaining_steps > 0 {

        // update particles here
        remaining_steps -= 1;
    }

    return simulator
}

impl ParticleSimulator {

    pub fn add_particle(&self) -> &ParticleSimulator {

        return self
    }

    pub fn remove_particle(&mut self, id: &u32) -> &ParticleSimulator {

        self.particles.remove(&id);
        return self
    }

    pub fn step(&self, steps: u32) -> &ParticleSimulator {


        advance_simulation(self, steps);

        return self
    }
    
}


/*
    Unittests
*/

#[cfg(test)]
mod tests {
    use crate::initialize_simulation;

    #[test]
    fn test_initialize() {
        let sim = initialize_simulation(0.01);

        assert!(sim.step_size == 0.01);

    }

}
