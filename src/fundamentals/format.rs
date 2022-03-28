#![allow(unused)]

use std::fmt::*;

#[derive(Debug)] // Display can not be derived
struct Hello {
    name: String,
    age: i32,
}

fn hello() {
    let hello = Hello {
        name: String::from("Abhinav"),
        age: 10,
    };
    println!("{:#?}", hello);
    println!("{:?}", hello);
}

/*---------------------------------------------------- */

#[derive(Debug)]
struct Numbers(i32, i32);

impl Display for Numbers {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, " Implemented ({} + {})", self.0, self.1)
    }
}

fn numbers() {
    let num = Numbers { 0: 4, 1: 5 }; // Numbers(4,5)
    println!("{}", num);
    println!("{:?}", num);
}

/*---------------------------------------------------- */

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Binary for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, " Binary Implemented ({} + {})", self.x, &self.y)
    }
}

fn point() {
    let pt = Point { x: 6.52, y: 5.5 };
    println!("{:b}", pt);
    println!("{:?}", pt);
}

/*---------------------------------------------------- */

#[derive(Debug)]
struct List(Vec<String>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, v)?;
        }

        // Close the opened bracket and return a Result value.
        write!(f, "]")
    }
}

fn list() {
    let fruits = List(vec![
        "Strawberry".to_string(),
        "Banana".to_string(),
        "Apple".to_string(),
        "Guava".to_string(),
    ]);
    println!("{:?}", fruits);
    println!("{}", fruits);
}

/*---------------------------------------------------- */

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "RGB ({red},{green},{blue}) 0x{:x}{:x}{:x}",
            red = self.red,
            green = self.green,
            blue = self.blue
        )
    }
}

fn color() {
    let mut colors: Vec<Color> = Vec::new();

    colors.push(Color {
        red: 255,
        blue: 100,
        green: 134,
    });

    colors.push(Color {
        red: 128,
        green: 255,
        blue: 90,
    });

    colors.push(Color {
        red: 255,
        blue: 78,
        green: 45,
    });

    println!("{:?}", colors);

    for color in colors.iter() {
        println!("{}", color);
    }
}

/*---------------------------------------------------- */

pub fn run() {
    println!("*****Format*****");
    println!();

    hello();
    numbers();
    point();
    list();

    color();

    println!("{:05o}", 69);
    println!("{:08X}", 10i32);
    eprintln!("This is error");

    let form = format!(
        "This
    is 
    great
    "
    );
    println!("{}", form)
}
