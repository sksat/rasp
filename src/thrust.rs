pub struct Thrust {
    pub curve: Vec<ThrustPoint>,
}

pub struct ThrustPoint {
    pub time: f64,
    pub thrust: f64,
}

impl Thrust {
    pub fn new() -> Self {
        Thrust {
            curve: Vec::<ThrustPoint>::new(),
        }
    }

    pub fn push_point(&mut self, time: f64, thrust: f64) {
        self.curve.push(ThrustPoint { time, thrust });
    }

    pub fn duration(&self) -> f64 {
        // sec
        todo!();
    }

    pub fn average(&self) -> f64 {
        // N
        todo!();
    }

    pub fn total_impulse(&self) -> f64 {
        // Ns
        todo!();
    }

    pub fn peak(&self) -> f64 {
        // N
        todo!();
    }
}
