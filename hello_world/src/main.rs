// fn main_hello_world() {
//     println!("Hello, world!");

//     println!("Base 10:               {}",   69420); // 69420
//     println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
//     println!("Base 8 (octal):        {:o}", 69420); // 207454
//     println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
// }

// use std::fmt; // Import `fmt`
// // A structure holding two numbers. `Debug` will be derived so the results can
// // be contrasted with `Display`.
// #[derive(Debug)]
// struct MinMax(i64, i64);

// #[derive(Debug)]
// struct Real(f64);
// #[derive(Debug)]
// struct Imag(f64);
// #[derive(Debug)]
// struct Complex {
//     real: Real, 
//     imag: Imag
// }


// // Implement `Display` for `MinMax`.
// impl fmt::Display for MinMax {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "({}, {})", self.0, self.1)
//     }
// }

// // Implement `Display for `Real`.`
// impl fmt::Display for Real {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
//         write!(f, "{}", self.0)
//     }
// }

// // Implement `Display for `Imag`.`
// impl fmt::Display for Imag {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
//         write!(f, "{}i", self.0)
//     }
// }

// // Implement `Display for `Complex`.`
// impl fmt::Display for Complex {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
//         write!(f, "{} + {}", self.real, self.imag)
//     }
// }
// // Define a structure where the fields are nameable for comparison.
// #[derive(Debug)]
// struct Point2D {
//     x: f64,
//     y: f64,
// }

// // Similarly, implement `Display` for `Point2D`.
// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Customize so only `x` and `y` are denoted.
//         write!(f, "x: {}, y: {}", self.x, self.y)
//     }
// }

// fn main() {
//     let minmax = MinMax(0, 14);

//     println!("Compare structures:");
//     println!("Display: {}", minmax);
//     println!("Debug: {:?}", minmax);

//     let big_range =   MinMax(-300, 300);
//     let small_range = MinMax(-3, 3);

//     println!("The big range is {big} and the small is {small}",
//              small = small_range,
//              big = big_range);

//     let point = Point2D { x: 3.3, y: 7.2 };

//     println!("Compare points:");
//     println!("Display: {}", point);
//     println!("Debug: {:?}", point);

//     // Error. Both `Debug` and `Display` were implemented, but `{:b}`
//     // requires `fmt::Binary` to be implemented. This will not work.
//     // println!("What does Point2D look like in binary: {:b}?", point);

//     let real_num: Real = Real(3.1);
//     let imag_num: Imag = Imag(7.7);
//     let complex_num: Complex = Complex{real: real_num, imag: imag_num};

//     println!("Compare complex numbers:");
//     println!("Display: {}", complex_num);
//     println!("Debug: {:?}", complex_num);
// }

// use std::fmt; // Import the `fmt` module.

// // Define a structure named `List` containing a `Vec`.
// struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Extract the value using tuple indexing,
//         // and create a reference to `vec`.
//         let vec = &self.0;

//         write!(f, "[")?;

//         // Iterate over `v` in `vec` while enumerating the iteration
//         // count in `count`.
//         for (count, v) in vec.iter().enumerate() {
//             // For every element except the first, add a comma.
//             // Use the ? operator to return on errors.
//             if count != 0 { write!(f, ", ")?; }
//             write!(f, "{}: {}", count, v)?;
//         }

//         // Close the opened bracket and return a fmt::Result value.
//         write!(f, "]")
//     }
// }

// fn main() {
//     let v = List(vec![1, 2, 3]);
//     println!("{}", v);
// }

use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
//# == "pretty print"
//RGB (0, 0, 0) 0x000000 - :0>2x == pad 2 0s (0>2) AND hexadecimal (x)
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGV ({}, {}, {}) 0x{:0>2x}{:0>2x}{:0>2x}", self.red, self.green, self.blue, self.red, self.green, self.blue)
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}