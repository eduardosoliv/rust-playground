use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c: char = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c: char = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let rgb: u32 = (self.r as u32 * 65536) + (self.g as u32 * 256) + self.b as u32;
        write!(f, "RGB ({} {} {}) 0x{:06X}", self.r, self.g, self.b, rgb)
    }
}

fn main() {
    cities();
    println!("");
    colors();
}

fn cities() {
    let cities: [City; 3] = [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ];

    for city in cities {
        println!("{}", city);
    }
}

fn colors() {
    let colors: [Color; 3] = [
        Color {
            r: 128,
            g: 255,
            b: 90,
        },
        Color { r: 0, g: 3, b: 254 },
        Color { r: 0, g: 0, b: 0 },
    ];
    for color in colors {
        println!("{}", color);
    }
}
