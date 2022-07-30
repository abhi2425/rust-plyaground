#![allow(unused)]

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }

pub fn run() {
    iterators();
    println!("// ------------------------------------------------------------------------ //");

    consumer_methods();
    println!("// ------------------------------------------------------------------------ //");

    adapter_methods();
    println!("// ------------------------------------------------------------------------ //");

    custom_iterator();
    println!("// ------------------------------------------------------------------------ //");

    custom_counter();
    println!("// ------------------------------------------------------------------------ //");

    fibonacci()
}

// ------------------------------------------------------------------------ //

/*  let vec = !vec[1,2,3];
   1) vec.iter() -> generates iterator with a reference (&1) ,(&2), (&3)
   2) vec.iter_mut() -> generates iterator with a mutable reference (&mut 1),(&mut 2), (&mut 3)
   3) vec.into_iter()-> generates iterator with owned value (1),(2),(3)

   ->  if we have used iter_mut() once on a collection then we can not use it again as well as can't  use iter(),
        into_iter() as we can have only one mutable reference that's it.

    ->  into_iter() moves the ownership, so we can not use iter() or iter_mut() further on same collection i.e it
        can be used only once.  {This does not applies for arrays coz they implement the "Copy" trait}

    ->  However we can use iter() multiple times as references to the values can be borrowed multiple times i.e (read-only)


*/

fn iterators() {
    let mut vec1 = vec![1, 2, 3, 4, 5];
    let mut arr1 = [1, 2, 3, 4, 4, 5];
    let mut arr2 = ["potatoes", "Spinach"];

    // let mut vec_iter_mut = vec1.iter_mut();
    // let mut vec_iter= vec1.iter();
    let mut vec_into_iter = vec1.into_iter();

    let mut arr_into_iter = arr1.into_iter();
    let mut arr_iter = arr1.iter();
    let mut arr2_iter_mut = arr2.iter_mut();

    // consumes arr2 and can't be used further
    // for elem in arr2 {
    //     println!("{}", elem);
    // }
    assert_eq!(arr_iter.next(), Some(&1));
    assert_eq!(arr_iter.next(), Some(&2));
    assert_eq!(arr_iter.next(), Some(&3));

    assert_eq!(arr2_iter_mut.next(), Some(&mut "potatoes"));
    assert_eq!(arr2_iter_mut.next(), Some(&mut "Spinach"));

    assert_eq!(arr_into_iter.next(), Some(1));
    assert_eq!(arr_into_iter.next(), Some(2));
    assert_eq!(arr_into_iter.next(), Some(3));

    // assert_eq!(vec_iter_mut.next(), Some(&mut 1));
    // assert_eq!(vec_iter_mut.next(), Some(&mut 2));
    // assert_eq!(vec_iter_mut.next(), Some(&mut 3));

    // assert_eq!(vec_into_iter.next(), Some(1));
    // assert_eq!(vec_into_iter.next(), Some(2));
    // assert_eq!(vec_into_iter.next(), Some(3));

    // assert_eq!(vec_iter.next(), Some(&1));
    // assert_eq!(vec_iter.next(), Some(&2));
    // assert_eq!(vec_iter.next(), Some(&3));

    println!("{:?}", arr2);
}

// ------------------------------------------------------------------------ //

fn consumer_methods() {
    // Method that takes in an iterator type and return result in different type
    let v1 = vec![1, 2, 3];
    let v2 = vec!["Abhinav", "Jha"];

    let v1_iter = v1.iter();
    let v2_iter = v2.iter();

    let total = v1_iter.sum::<i32>();
    let join: &[&str] = v2_iter.as_slice();

    // println!("{:?}",v1_iter);  // can't use v1_iter as its already consumed by sum
    println!("v1-> {:?}", v1);
    println!("v2-> {:?}", v2);
    // v2;
    // v1;

    println!("total-:{:?}", total);
    println!("Join-:{:?}", join);
}

// ------------------------------------------------------------------------ //

fn adapter_methods() {
    // Method that takes in an iterator type and return result as an iterator
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    //Calling the map method to create a new iterator and then calling the collect method to consume the new iterator and create a vector

    assert_eq!(v2, vec![2, 3, 4]);
    println!("{:?}", v2)
}

// ------------------------------------------------------------------------ //

fn custom_iterator() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u8,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u8) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        // we are using into_iter() because we need to pass the ownership as we are returning from the function
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );

    println!("{:?}", in_my_size)
}

// ------------------------------------------------------------------------ //

#[derive(Debug)]
struct Counter {
    count: i32,
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count > 20 {
            None
        } else {
            Some(self.count)
        }
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl Counter {
    fn new(count: i32) -> Self {
        Self { count }
    }
}

fn custom_counter() {
    let mut counter = Counter::new(0);
    let counter2 = Counter::new(0);

    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());

    while (counter.next() != None) {
        println!("{}", counter.count);
    }
    for elem in counter2.skip(10).take(5) {
        println!("For Loop-: {}", elem)
    }
}

fn fibonacci() {
    struct Fibonacci {
        current: u32,
        next: u32,
    }

    // Implement `Iterator` for `Fibonacci`.
    // The `Iterator` trait only requires a method to be defined for the `next` element.
    impl Iterator for Fibonacci {
        // We can refer to this type using Self::Item
        type Item = u32;
        // Here, we define the sequence using `.current` and `.next`.
        // The return type is `Option<T>`:
        //     * When the `Iterator` is finished, `None` is returned.
        //     * Otherwise, the next value is wrapped in `Some` and returned.
        // We use Self::Item in the return type, so we can change
        // the type without having to update the function signatures.
        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.current + self.next;

            self.current = self.next;
            self.next = new_next;

            // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
            // will never return `None`, and `Some` is always returned.
            Some(self.current)
        }
    }

    // Returns a Fibonacci sequence generator
    fn fibonacci() -> Fibonacci {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {i}");
    }
}
