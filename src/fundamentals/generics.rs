#![allow(unused)]
pub fn run() {
    generics();
    implementations();
    generic_trait();
    generic_bound();
    empty_bound();
    multiple_bound();
}

// --------------------------------------------------------- //

// A concrete type `A`.

fn generics() {
    struct A;

    // In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
    // Therefore, `Single` is a concrete type, and `A` is defined as above.
    struct Single(A);
    //            ^ Here is `Single`s first use of the type `A`.

    // Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
    // Because the type parameter `T` is generic, it could be anything, including
    // the concrete type `A` defined at the top.
    struct SingleGen<T>(T);

    // `Single` is concrete and explicitly takes `A`.
    let _s = Single(A);
    // Create a variable `_char` of type `SingleGen<char>`
    // and give it the value `SingleGen('a')`.
    // Here, `SingleGen` has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified:
    let _t = SingleGen(A); // Uses `A` defined at the top.
    let _i32 = SingleGen(6); // Uses `i32`.
    let _char = SingleGen('a'); // Uses `char`.
}

// --------------------------------------------------------- //

fn implementations() {
    struct Val {
        val: f64,
    }

    struct GenVal<T> {
        gen_val: T,
    }

    // impl of Val
    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    // impl of GenVal for a generic type `T`
    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    let name = GenVal { gen_val: "Abhinav" };

    println!("{}, {}, {}", x.value(), y.value(), name.value());
}

// --------------------------------------------------------- //

fn generic_trait() {
    // Non-copyable types.
    struct Empty;
    struct Null;

    // A trait generic over `T`.
    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an
        // additional single parameter `T` and does nothing with it.
        fn double_drop(self, _: T);
    }

    // Implement `DoubleDrop<T>` for any generic parameter `T` and
    // caller `U`.
    impl<T, U> DoubleDrop<T> for U {
        // This method takes ownership of both passed arguments,
        // deallocating both.
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null = Null;

    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    // empty;
    //null;
    // ^ TODO: Try uncommenting these lines.
}

// --------------------------------------------------------- //

fn generic_bound() {
    // A trait which implements the print marker: `{:?}`.
    use std::fmt::Debug;

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }
    #[allow(dead_code)]
    struct Triangle {
        length: f64,
        height: f64,
    }

    // The generic `T` must implement `Debug`. Regardless
    // of the type, this will work properly.
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    // `T` must implement `HasArea`. Any type which meets
    // the bound can access `HasArea`'s function `area`.
    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());
    println!("{}", area(&rectangle));
    // print_debug(&_triangle);
    //println!("Area: {}", _triangle.area());
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`.
}
// --------------------------------------------------------- //

fn empty_bound() {
    struct Cardinal;
    struct BlueJay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red for Cardinal {}
    impl Blue for BlueJay {}

    // These functions are only valid for types which implement these
    // traits. The fact that the traits are empty is irrelevant.
    fn red<T: Red>(_: &T) -> &str {
        "red"
    }
    fn blue<T: Blue>(_: &T) -> &'static str {
        "blue"
    }

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));
}

// --------------------------------------------------------- //

fn multiple_bound() {
    use std::fmt::{Debug, Display};

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array);
    // TODO ^ Try uncommenting this.

    compare_types(&array, &vec);
}

// --------------------------------------------------------- //

fn where_clause() {
    use std::fmt::Debug;

    trait PrintInOption {
        fn print_in_option(self);
    }

    // Because we would otherwise have to express this as `T: Debug` or
    // use another method of indirect approach, this requires a `where` clause:
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        // We want `Option<T>: Debug` as our bound because that is what's
        // being printed. Doing otherwise would be using the wrong bound.
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];

    vec.print_in_option();
}

// --------------------------------------------------------- //

fn new_type_idiom() {
    struct Years(i64);

    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        /// truncates partial years
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
}
