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
        let last = &self.curve.last().unwrap();
        last.time
    }

    pub fn average(&self) -> f64 {
        // N
        self.total_impulse() / self.duration()
    }

    pub fn total_impulse(&self) -> f64 {
        // Ns
        let mut t1: f64 = 0.0;
        let mut f1: f64 = 0.0;
        let mut sum: f64 = 0.0;
        for t in &self.curve {
            sum += (t.time - t1) * (f1 + t.thrust) * 0.5;
            t1 = t.time;
            f1 = t.thrust;
        }
        sum
    }

    pub fn peak(&self) -> f64 {
        // N
        let mut peak: f64 = 0.0;
        for t in &self.curve {
            if t.thrust > peak {
                peak = t.thrust
            }
        }
        peak
    }
}
