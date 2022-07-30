#![allow(unused)]
pub fn run() {
    lifetimes();
    println!("// ---------------------------------------------------------------------------------------------- //");

    explicit_annotation();
    println!("// ---------------------------------------------------------------------------------------------- //");

    struct_lifetime_annotation();
    println!("// ---------------------------------------------------------------------------------------------- //");

    trait_bound_lifetime();
    println!("// ---------------------------------------------------------------------------------------------- //");

    static_lifetime_coercion();
    println!("// ---------------------------------------------------------------------------------------------- //");

    elision();
}

/*
1) Each parameter that is a reference get it's own lifetime.

2) If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime
    parameter.

3  If there are multiple input lifetime parameter, but one of them is &self or &mut self, the lifetime of
    self is assigned to all output lifetime parameters
*/

// ---------------------------------------------------------------------------------------------- //

// lifetime is generally used for borrowed or referenced values
fn lifetimes() {
    // Lifetimes are annotated below with lines denoting the creation
    // and destruction of each variable.
    // `i` has the longest lifetime because its scope entirely encloses
    // both `borrow1` and `borrow2`. The duration of `borrow1` compared
    // to `borrow2` is irrelevant since they are disjoint.
    let i = 3; // Lifetime for `i` starts. ────────────────┐
               //                                          │
    {
        //                                                 │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
                          //                              ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 ends. ──────────────────────────────────┘│
      //                                                   │
      //                                                   │
    {
        //                                                 │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
                          //                              ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
      //                                                   │
} // Lifetime ends.   ─────────────────────────────────────┘

// ---------------------------------------------------------------------------------------------- //

fn explicit_annotation() {
    fn longest_str<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if (str1.len() > str2.len()) {
            str1
        } else {
            str2
        }
    }

    /**
     * Case -: 1
     */
    let result: &str;
    let a = String::from("Abhinav Jha");
    {
        let b = String::from("This is the larger string");
        result = longest_str(a.as_str(), b.as_str());
        println!("Longest string is-:{}", &result);
    }
    // println!("Longest string is-:{}", &result);

    /**
     * Case -: 2
     */
    let a = "Samuel Miranda";
    let result: &str;
    {
        let b = "Second Biggest String ";
        result = longest_str(a, b); // longest_str(&a, &b);
    }
    println!("Second Longest string is-:{}", &result);
}

// ---------------------------------------------------------------------------------------------- //

fn struct_lifetime_annotation() {
    // A type `Borrowed` which houses a reference to an
    // `i32`. The reference to `i32` must outlive `Borrowed`.
    #[derive(Debug)]
    struct Borrowed<'a, T>(&'a T);

    // Similarly, both references here must outlive this structure.
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    // An enum which is either an `i32` or a reference to one.
    #[derive(Debug)]
    enum Either<'a, 'b> {
        Num(&'b i32),
        Ref(&'a i32),
    }

    let x = 18;
    let y = 15;

    let single = Borrowed::<i32>(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(&y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

// ---------------------------------------------------------------------------------------------- //

fn trait_bound_lifetime() {
    use std::fmt::Debug; // Trait to bound with.

    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);
    // `Ref` contains a reference to a generic type `T` that has
    // an unknown lifetime `'a`. `T` is bounded such that any
    // *references* in `T` must outlive `'a`. Additionally, the lifetime
    // of `Ref` may not exceed `'a`.

    // A generic function which prints using the `Debug` trait.
    fn print<T>(t: T)
    where
        T: Debug,
    {
        println!("`print`: t is {:?}", t);
    }

    // Here a reference to `T` is taken where `T` implements
    // `Debug` and all *references* in `T` outlive `'a`. In
    // addition, `'a` must outlive the function.
    fn print_ref<'a, T>(t: &'a T)
    where
        T: Debug + 'a,
    {
        println!("`print_ref`: t is {:?}", t);
    }

    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);

    // -----------------------------------------------//

    fn print_it(input: impl Debug + 'static) {
        println!("'static value passed in is: {:?}", input);
    }
    fn add(&i: &i32) -> i32 {
        i
    }

    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    // print_it(&i);
    add(&i);
}

// ---------------------------------------------------------------------------------------------- //

// 'static as part of a trait bound:
fn generic<T>(x: T)
where
    T: 'static,
{
}

// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

fn static_lifetime_coercion() {
    // A reference with 'static lifetime:
    let s: &'static str = "hello world";

    // Returns a reference to `NUM` where its `'static`
    // lifetime is coerced to that of the input argument.
    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

// ---------------------------------------------------------------------------------------------- //

fn elision() {
    // `elided_input` and `annotated_input` essentially have identical signatures
    // because the lifetime of `elided_input` is inferred by the compiler:
    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x);
    }

    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x);
    }

    // Similarly, `elided_pass` and `annotated_pass` have identical signatures
    // because the lifetime is added implicitly to `elided_pass`:
    fn elided_pass(x: &i32) -> &i32 {
        x
    }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
