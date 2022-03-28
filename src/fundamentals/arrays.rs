#![allow(unused)]

use std::mem::size_of_val;

fn iter() {
    let v = vec![1, 2, 3, 4];

    for elem in v.iter() {

        // borrows v and &elem
    }

    println!(" can be printed {:?}", v);

    for elem in &v {

        // borrows v and &elem
    }
    println!(" can be printed {:?}", v);

    for elem in v {
        // consumes v, owned elem
    }

    // println!(" can not be printed {:?}", v);
}

pub fn run() {
    let mut fruits: [&str; 4] = ["Banana", "Apple", "Pear", "Guava"];
    let veggies = ["potatoes", "Spinach", "Brocoli", "cabbage", "Cauliflower"];
    let roughages = ["Cucumber", "Radish", "Carrot"];

    println!(
        "fruits-: {:?}, length-: {} Size- {}",
        fruits,
        fruits.len(),
        size_of_val(&fruits)
    );

    get_characters(&fruits);
    fruit_juice(&mut fruits);
    get_slice(&veggies[0..2])
}

fn fruit_juice(fruit: &mut [&str; 4]) {
    fruit[0] = "Orange";
    println!("{:?}", fruit)
}

fn get_slice(slice: &[&str]) {
    println!("Veggie Slice=>{:?}", slice)
}

fn get_characters(fruits: &[&str; 4]) {
    for (i, name) in fruits.iter().enumerate() {
        println!("{}", name)
    }

    let merged = fruits
        .iter()
        .map(|f| f.chars())
        .flatten()
        .collect::<String>();

    let chars = merged.chars().collect::<Vec<char>>();
    println!("{:?}", chars)
}
