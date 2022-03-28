#![allow(unused)]
pub fn run() {
    if_else();
    if_let();
    while_let();
    use_loop();
    use_while();
    use_for();
}

fn if_else() {
    let n = 5i32;
    println!(" <--------------If Else------------>");

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        // This expression must return an `i32` as well.
        n / 2
        // TODO ^ Try suppressing this expression with a semicolon.
    };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n)
}

// ---------------------------------------------------------- //
fn if_let() {
    // All have type `Option<i32>`
    let number = Some(7);
    let letter = None::<i32>;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructure `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // ------------------------------------------------------ //
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    };

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}

fn while_let() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructure `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

// ---------------------------------------------------------- //

fn use_loop() {
    let mut count = 0;
    println!(" <--------------Loop------------>");
    loop {
        count += 1;
        if count == 3 {
            println!("Skip");
            continue;
        }

        if (count == 5) {
            println!(" That's enough");
            break;
        }
        println!("{}", count);
    }
    println!(" <--------------Loop Labels------------>");

    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // This would break only the inner loop
            //break;
            // This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    println!(" <--------------Loop Return Value------------>");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result-{} Counter-{}", result, counter);
}

// ---------------------------------------------------------- //

fn use_while() {
    // A counter variable
    let mut n = 1;

    println!(" <--------------While Loop----------->");

    // Loop while `n` is less than 101
    while n < 50 {
        if n % 15 == 0 {
            println!("fizz_buzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}

// ---------------------------------------------------------- //

fn use_for() {
    println!(" <--------------For Loop------------>");

    for n in 1..=50 {
        if n % 15 == 0 {
            println!("fizz-buzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in &names {
        // will borrow the value
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    for name in names.iter() {
        // will borrow the value
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    for name in names.into_iter() {
        // will move the value in the loop that is taking the ownership
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names); // "names" cannot be used as it is moved to for loop

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
