pub mod thrust;

pub struct Engine {
    pub name: String,
    pub diameter: u32,           // mm
    pub length: u32,             // mm
    pub delay: Option<[u32; 8]>, // sec
    pub weight_propellant: f64,  // kg
    pub weight_init: f64,        // kg
    pub manufacturer: String,
    pub thrust: thrust::Thrust,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error {}

pub fn from_str(s: &str) -> Result<Engine, Error> {
    let line: Vec<&str> = s.split("\n").collect();

    // skip comment line
    let mut li = line.iter().skip_while(|l| l.chars().nth(0) == Some(';'));

    // this line must be header
    let header = li.next();
    //println!("header: {}", header.unwrap());
    let header: Vec<_> = header.unwrap().split_whitespace().collect();
    let name = header[0].to_string();
    let diameter = header[1].parse().unwrap();
    let length = header[2].parse().unwrap();
    let delay = match header[3] {
        "0" => None,
        _ => unimplemented!("delays"),
    };
    let weight_propellant = header[4].parse().unwrap();
    let weight_init = header[5].parse().unwrap();
    let manufacturer = header[6].to_string();

    let mut thrust = thrust::Thrust::new();
    while let Some(l) = li.next() {
        let l: Vec<_> = l.split_whitespace().collect();
        let time = l[0];
        let data = l[1];
        thrust.push_point(time.parse().unwrap(), data.parse().unwrap());
    }

    let eng = Engine {
        name,
        diameter,
        length,
        delay,
        weight_propellant,
        weight_init,
        manufacturer,
        thrust,
    };
    Ok(eng)
}

impl std::fmt::Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        let h0 = format!("Name: {}({})", self.name, self.manufacturer);
        let h1 = format!("Diameter: {}mm, Length: {}mm", self.diameter, self.length);
        let w = format!(
            "Weight: total(init)={}kg, propellant={}kg({:.2}%)",
            self.weight_init,
            self.weight_propellant,
            self.weight_propellant / self.weight_init * 100.0
        );
        let t = format!(
            "Thrust: Duration={:.2}s, Total Impulse={:.2}Nt-s, Average={:.2}N, Peak={:.2}N",
            self.thrust.duration(),
            self.thrust.total_impulse(),
            self.thrust.average(),
            self.thrust.peak()
        );

        use std::cmp::max;
        let l = max(
            max(h0.chars().count(), h1.chars().count()),
            max(w.chars().count(), t.chars().count()),
        );

        let _ = write!(f, "{}\n{}\n{}\n{}\n", h0, h1, w, t);
        let _ = write!(f, "{}\n", "-".repeat(l));
        let _ = write!(f, "Time(sec) Thrust(N)\n");

        for t in &self.thrust.curve {
            let _ = write!(f, "{} {}\n", t.time, t.thrust);
        }

        write!(f, "\n")
    }
}
