use crate::week1::{count_chars, string_join, trim, merge_sort};
use rand::{prelude::*, distributions::Uniform};


mod week1;

fn main() {
    println!("Hello, world!");
    println!("week1 count_chars {}", count_chars::main("foobaro", 'o'));
    println!("week1 string_join {}", string_join::main(vec![String::from("foo"), String::from("bar"), String::from("lol")]));
    println!("week1 trim {}", trim::main(String::from("  foobar       ")));
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 10000);

    let mut vec: Vec<i32> = (0..50).map(|_| rng.sample(&range)).collect();
    merge_sort::main(&mut vec);
    println!("week1 merge_sort {:?}", vec)
}
