#![allow(unused)]

pub fn run() {
    req_acq_is_init();
    ownership();
    mutability_shift();
}

fn req_acq_is_init() {
    fn create_box() {
        // Allocate an integer on the heap
        let _box1 = Box::new(3i32);

        // `_box1` is destroyed here, and memory gets freed
    }

    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}
// --------------------------------------------------------- //

fn ownership() {
    // This function takes ownership of the heap allocated memory
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);

        // `c` is destroyed and the memory freed
    }

    // _Stack_ allocated integer
    let x = 5u32;
    let name1 = "abhinav";

    // *Copy* `x` into `y` - no resources are moved
    let y = x;
    let name2 = name1;

    // Both values can be independently used
    println!("x is {}, and y is {} ", x, y);
    println!("Name-1 is {}, and Name-2 is {} ", name1, name2);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.
    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    //println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    // println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
}

// --------------------------------------------------------- //

fn mutability_shift() {
    // Mutability of data can be changed when ownership is transferred.

    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
