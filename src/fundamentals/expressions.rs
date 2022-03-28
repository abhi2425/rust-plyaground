#![allow(unused)]
pub fn run() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    let hello = {
        fn test() -> i32 {
            println!("hello from block");
            6
        }
        test()
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
    println!("Hello Var is {:?}", hello);
}
