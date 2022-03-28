#![allow(unused)]

mod fundamentals;

fn main() {
    // fundamentals::run_vars();
    // fundamentals::run_format();
    // fundamentals::run_literals_operators();
    // fundamentals::run_tuples();
    // fundamentals::run_arrays();
    // fundamentals::run_structs();
    // fundamentals::run_types();
    // fundamentals::run_conversion();
    // fundamentals::run_expressions();
    // fundamentals::run_flow_control();
    // fundamentals::run_flow_match();
    // fundamentals::run_closures();
    // fundamentals::run_modules();
    // fundamentals::run_generics();
    // fundamentals::run_associated_types();
    fundamentals::run_scoping_rules();
    // cfg();
}

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn cfg() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
