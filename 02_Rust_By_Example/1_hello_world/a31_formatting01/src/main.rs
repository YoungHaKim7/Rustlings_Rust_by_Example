use std::fmt::{self, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{} {:.3}°{} {:.3}° {} ",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City {
            name: "London",
            lat: 51.5073,
            lon: -0.1271,
        },
        City {
            name: "Paris",
            lat: 48.8566,
            lon: 2.3522,
        },
        City {
            name: "Berlin",
            lat: 52.5236,
            lon: 13.4056,
        },
    ] {
        println!("{}", city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
    ] {
        println!("{:?}", color);
    }
}
