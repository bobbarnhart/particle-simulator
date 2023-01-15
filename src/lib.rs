use pyo3::prelude::*;
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

#[pyclass]
struct ParticleSimulator {
    step_size: f32,
    time_seconds: f32,
    particles: Vec<Particle>,
}

fn initialize_simulation(step_size: f32) -> ParticleSimulator {
    let time_seconds = 0.0;
    let particles: Vec<Particle> = Vec::new();
    let simulator = ParticleSimulator {
        step_size: step_size,
        time_seconds: time_seconds,
        particles: particles,
    };

    return simulator;
}

fn get_side_by_string(side: String) -> Result<Side, String> {
    match side.to_uppercase().as_str() {
        "BLUE" => Ok(Side::BLUE),
        "RED" => Ok(Side::RED),
        "GREEN" => Ok(Side::GREEN),
        "YELLOW" => Ok(Side::YELLOW),
        _ => return Err("unrecognized string".to_string())
    }
}

fn advance_simulation(simulator: &ParticleSimulator, steps: u32) -> &ParticleSimulator {
    let mut remaining_steps: u32 = steps;

    while remaining_steps > 0 {
        // update particles hereS
        remaining_steps -= 1;
    }

    return simulator;
}

#[pymethods]
impl ParticleSimulator {

    #[new]
    pub fn new(step_size: f32) -> ParticleSimulator {
        return initialize_simulation(step_size);
    }

    pub fn add_particle(&mut self, name: String, side: String, radius: f32, position: [f32; 3], velocity: [f32; 3]) {
        let side_enum: Side = get_side_by_string(side).unwrap();
        let new_particle: Particle = Particle { name: name, side: side_enum, radius: radius, position: position, velocity: velocity };
        let new_index: usize = self.particles.len();
        self.particles.insert(new_index, new_particle);
    }

    pub fn remove_particle(&mut self, id: usize) {
        self.particles.remove(id);
    }

    pub fn step(&mut self, steps: u32) {
        advance_simulation(self, steps);
    }

    pub fn reset(&mut self) {

        self.particles.clear();
        self.time_seconds = 0.0;
    }
}

#[pymodule]
fn particle_simulator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ParticleSimulator>()?;
    Ok(())
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

        sim.add_particle(name, String::from("blue"), radius, position, velocity);
        sim.remove_particle(0);
        assert!(sim.particles.len() == 0);
    }
}
