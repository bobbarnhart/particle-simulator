
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
    velocity: [f32; 3],
}
struct ParticleSimulator {
    step_size: f32,
    time_seconds: f32,
    particles: Vec<Particle>,
    particle_index: usize,
}

fn add_particle(mut simulator: ParticleSimulator, particle: Particle) -> ParticleSimulator {
    // add a new particle to the simulator
    simulator.particles.insert(simulator.particles.len(), particle);
    return simulator;
}

fn initialize_simulation(step_size: f32) -> ParticleSimulator {
    let time_seconds = 0.0;
    let index: usize = 0;
    let particles: Vec<Particle> = Vec::new();
    let simulator = ParticleSimulator {
        step_size: step_size,
        time_seconds: time_seconds,
        particles: particles,
        particle_index: index,
    };

    return simulator;
}

fn advance_simulation(simulator: &ParticleSimulator, steps: u32) -> &ParticleSimulator {
    let mut remaining_steps: u32 = steps;

    while remaining_steps > 0 {
        // update particles hereS
        remaining_steps -= 1;
    }

    return simulator;
}

impl ParticleSimulator {
    pub fn add_particle(&mut self, particle: Particle) -> &ParticleSimulator {
        let new_index: usize = self.particle_index;
        self.particles.insert(new_index, particle);
        self.particle_index += 1;
        return self;
    }

    pub fn remove_particle(&mut self, id: usize) -> &ParticleSimulator {
        self.particles.remove(id);
        return self;
    }

    pub fn step(&mut self, steps: u32) -> &ParticleSimulator {
        return advance_simulation(self, steps);
    }

    pub fn reset(&mut self) -> &ParticleSimulator {

        self.particles.clear();
        self.particle_index = 0;
        self.time_seconds = 0.0;
        return self
    }
}

/*
    Unittests
*/

#[cfg(test)]
mod tests {
    use crate::{initialize_simulation, Particle, Side};

    #[test]
    fn test_initialize() {
        let sim = initialize_simulation(0.01);

        assert!(sim.step_size == 0.01);
    }

    #[test]
    fn test_add_remove_particle() {
        /*
            Make sure that we can add and remove particles
            within the simulation
        
        */
        let mut sim = initialize_simulation(0.01);

        let name: String = String::from("my_particle");
        let position: [f32; 3] = [0.0, 1.0, 0.0];
        let velocity: [f32; 3] = [1.0, 1.0, 0.0];
        let radius: f32 = 1.0;

        let particle = Particle {
            name: name,
            side: Side::BLUE,
            position: position,
            velocity: velocity,
            radius: radius,
        };

        sim.add_particle(particle);

        assert!(sim.particle_index == 1);

        sim.remove_particle(0);

        assert!(sim.particles.len() == 0);
    }
}
