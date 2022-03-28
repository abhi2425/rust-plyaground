#![allow(unused)]
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn run() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    let int = 5;
    let num = Number::from(30);
    let num2: Number = int.into();
    println!("My number is {:?}", num);
    println!("Num2 -{:?}", num2);

    from_into();
    to_string();
    parse_string();
}
// --------------------------------------------------------------//

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            return Err({});
        }
    }
}
// --------------------------------------------------------- //
fn from_into() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    EvenNumber::try_from(9);
    println!("EvenNumber Error: {:?}", EvenNumber::try_from(9));
    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

// --------------------------------------------------------- //

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn to_string() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

// --------------------------------------------------------- //

fn parse_string() {
    let parsed = match "5a".parse::<u32>() {
        Ok(value) => value,
        Err(v) => 0,
    };
    let turbo_parsed = "10".parse::<u32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
