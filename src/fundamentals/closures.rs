#![allow(unused)]
pub fn run() {
    println!("// ---------------------------------------------------------- //");
    closures();
    println!("// ---------------------------------------------------------- //");
    capture_closure();
    println!("// ---------------------------------------------------------- //");
    closure_as_input();
    println!("// ---------------------------------------------------------- //");
    closure_as_input_function();
    println!("// ---------------------------------------------------------- //");
    closure_as_output();
    println!("// ---------------------------------------------------------- //");
    closure_examples()
}

fn closures() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}

// ------------------------------------------------------------------ //

/*
 * Closures can capture variables:

-> by reference: &T
-> by mutable reference: &mut T
-> by value: T
 */

fn capture_closure() {
    use std::mem;
    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _re_borrow = &color;
    print();

    // A move or re_borrow is allowed after the final use of `print`
    let _color_moved = color;
    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to re_borrow will lead to an error.
    // let _re_borrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to re_borrow without an error
    let _count_re_borrowed = &mut count;

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
}

// ------------------------------------------------------------------------------------- //

/*
When taking a closure as an input parameter, the closure's complete type must be annotated using one of a few traits, and they're determined by what the closure does with captured value. In order of decreasing restriction, they are:

-> Fn: the closure uses the captured value by reference (&T)
-> FnMut: the closure uses the captured value by mutable reference (&mut T)
-> FnOnce: the closure uses the captured value by value (T)

 */

fn closure_as_input() {
    use std::mem;

    // A function which takes a closure as an argument and calls it.
    // <F> denotes that F is a "Generic type parameter"
    fn apply<F>(f: F)
    where
        // The closure takes no input and returns nothing.
        F: FnOnce(),
    {
        // ^ TODO: Try changing this to `Fn` or `FnMut`.

        f();
    }

    // A function which takes a closure and returns an `i32`.
    fn apply_to_3<F>(f: F) -> i32
    where
        // The closure takes an `i32` and returns an `i32`.
        F: Fn(i32) -> i32,
    {
        f(3)
    }
    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

// ------------------------------------------------------------------------------------- //

fn closure_as_input_function() {
    // Define a function which takes a generic `F` argument
    // bounded by `Fn`, and calls it
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    // Define a wrapper function satisfying the `Fn` bound
    fn function() {
        println!("I'm a function!");
    }

    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

// ------------------------------------------------------------------------------------- //

fn closure_as_output() {
    fn create_fn() -> impl Fn() -> i32 {
        let text = "Fn".to_owned();

        move || -> i32 {
            println!("This is a: {}", text);
            4
        }
    }

    fn create_fn_mut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fn_once() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        return move || println!("This is a: {}", text);
    }

    let fn_plain = create_fn()();
    let mut fn_mut = create_fn_mut();
    let fn_once = create_fn_once();

    println!("{}", fn_plain);
    fn_mut();
    fn_once();
}

pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F: FnMut(Self::Item)>(&mut self, f: F) -> bool;
    fn any2(&mut self, f: dyn FnMut(Self::Item)) -> bool;
}

pub trait Iterator2 {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool;
}

// ------------------------------------------------------------------------------------- //

fn closure_examples() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vec yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vec yields `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` for vec yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&x| *x == 2));
    // `into_iter()` for vec yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|x| *x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays yields `i32`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );
}
