#![allow(unused)]

pub fn run() {
    traits();
    println!("// ------------------------------------------------------------------------ //");

    return_traits_with_dyn();
    println!("// ------------------------------------------------------------------------ //");

    operator_overloading();
    println!("// ------------------------------------------------------------------------ //");

    drop_trait();
    println!("// ------------------------------------------------------------------------ //");
}

// ------------------------------------------------------------------------ //

fn traits() {
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait Animal {
        // Associated function signature; `Self` refers to the implementor type.
        fn new(name: &'static str) -> Self;

        // Method signatures; these will return a string.
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // Traits can provide default method definitions.
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // Implementor methods can use the implementor's trait methods.
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        // `Self` is the implementor type: `Sheep`.
        fn new(name: &'static str) -> Sheep {
            Sheep {
                name,
                naked: false,
            }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }
        // Default trait methods can be overridden.
        fn talk(&self) {
            // For example, we can add some quiet contemplation.
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    // Type annotation is necessary in this case.
    let mut dolly = Sheep::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

// ------------------------------------------------------------------------ //

fn return_traits_with_dyn() {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        // Instance method signature
        fn noise(&self) -> &'static str;
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaaah!"
        }
    }

    // Implement the `Animal` trait for `Cow`.
    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "moooooo!"
        }
    }

    // Returns some struct that implements Animal, but we don't know which one at compile time.
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}

// ------------------------------------------------------------------------ //

fn operator_overloading() {
    use std::ops;

    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    // The `std::ops::Add` trait is used to specify the functionality of `+`.
    // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
    // The following block implements the operation: Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            println!("> Foo.add(Bar) was called");

            FooBar
        }
    }

    // By reversing the types, we end up implementing non-commutative addition.
    // Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
    // This block implements the operation: Bar + Foo = BarFoo
    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, _rhs: Foo) -> BarFoo {
            println!("> Bar.add(Foo) was called");

            BarFoo
        }
    }

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

// ------------------------------------------------------------------------ //

/*
   The Drop trait only has one method: drop, which is called automatically when an object goes out of scope. The main use of the Drop trait is to free the resources that the implementor instance owns.

   Box, Vec, String, File, and Process are some examples of types that implement the Drop trait to free resources. The Drop trait can also be manually implemented for any custom data type

*/

fn drop_trait() {
    struct Droppable {
        name: &'static str,
    }

    // This trivial implementation of `drop` adds a print to console.
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line

    println!("end of the function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed
}
