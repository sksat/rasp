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
    println!("header: {}", header.unwrap());
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
