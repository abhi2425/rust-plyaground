#![allow(unused)]

pub fn run() {
    let mut name = String::from("Abhinav Jha");
    let name2 = "Abhishek";
    let mut number: i16 = 45;

    println!("{}-{}-{}", name, name2, number);

    fun(&mut name);
    fun2(name2);
    increment(&mut number);
    println!("{} {} {}", name, name2, number);
}

fn scope() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
}

fn declare_first() {
    // Declare a variable binding
    let a_binding;
    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);
    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

fn freezing() {
    let mut _mutable_integer = 7i32;
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        // FIXME ^ Comment out this line

        // `_mutable_integer` goes out of scope
    }
    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}

fn fun(name: &mut String) {
    name.push_str(" is great");
    println!("fun Function calling name {}", name);
    *name = String::from("Samuel miranda");
    println!("fun Function calling name {}", name);
}

fn fun2(name: &str) {
    println!("fun2 Function calling name {}", name)
}

fn increment(num: &mut i16) {
    *num += 1;

    println!("Num-{} *Num-{}", num, *num)
}
