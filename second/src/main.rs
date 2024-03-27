#![allow(unused)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Range;
use std::ops::RangeInclusive;

fn check_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn divide(dividend: i32, divisor: i32) -> i32 {
    match check_division(dividend, divisor) {
        None => {
            println!("Division by zero!");
            0
        }
        Some(value) => value,
    }
}

fn main() {
    // let num:i32 = 12;

    // fn recursion(num:i32) {
    //     if num == 0 {
    //         return;
    //     }
    //     println!("{} ", num);
    //     return recursion(num - 1);
    // }
    // recursion(num);

    // let arr: Vec<i32> = vec![1, 2, 3, 4, 5];

    // for i in arr {
    //     if i == 3 {
    //         println!("{} ", i);
    //     }else {
    //         println!("searching... ");
    //     }
    // }
    // println!("Enter your name: ");
    // let mut names = String::new();
    // io::stdin().read_line(&mut names).expect("Failed to read line");
    // let mut trimmed_name = names.trim();
    // if trimmed_name.is_empty() {
    //     trimmed_name.to_string();
    //     trimmed_name= "user";
    // }
    // println!("Hello, {}", trimmed_name);

    let dividend = 10;
    let divisor = 4;
    let result = divide(dividend, divisor);
    println!("{:?} / {:#} = {}", dividend, divisor, result);
    // const PI: f32 = 3.14; // use const for global variable and immutable variables
    println!("The max value of u32 = {}", std::u32::MAX);
    println!("The max value of u64 = {}", std::u64::MAX);

    let mut vec: Vec<i32> = Vec::new();

    fn push(vec: &mut Vec<i32>) -> Vec<i32> {
        vec.push(1);
        vec.to_vec()
    }

    let result = push(&mut vec);
    println!("{:?}", result); // to print a vector we need to use {:?} instead of {} because std::fmt::Display is not implemented for Vec<T>

    // macro_rules! f_string {
    //     ($($tokens:tt)*) => {
    //         format!($($tokens)*)
    //     };
    // }

    let num1 = 21;
    let num2 = 22;
    // assert_eq!(num1, num2, "the two numbers are not equal");
    // println!("the two numbers are equal");

    let x: i32 = 10;
    let y;
    {
        y = 5;
        // println!("The value of x is {} and value of y is {}", x, y);
    }
    // println!("The value of x is {} and value of y is {}", x, y);

    // create a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = tup;
    println!("The value of tup2 is {:?}", tup);
    // add to the tuple
    let (x, y, z) = tup;
    println!("The value of x is {}, y is {} and z is {}", x, y, z);

    let v = 28_u8;

    {
        // this is shadowing: meaning that the code inside this block {} is not the same as the code outside in sense of not interferring with the code outside
        fn type_of<T>(value: &T) -> String {
            let a = std::any::type_name::<T>();
            println!("{}", a);
            a.to_string()
        }
        let x = 1_000.000_1;
        println!("{}", type_of(&x));
        let float1 = 0.1;
        let float2 = 0.2;
        let add_float: f32 = float1 + float2;
        let float3 = 0.3;
        add_float.abs();
        assert_eq!(add_float, float3);

        println!("Success!");
    }

    for i in 1..=10 {
        // this is a range inclusive
        println!("{}", i);
    }

    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // the ..= is a shorthand for range inclusive not to be mistaken for the .. which is a shorthand for range
    let cal = 24 % 5;
    println!("{}", cal);

    // assert!(true && false == false);

    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.into_bytes();
    _s.iter().for_each(|x| println!("{} ", x));

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
        gender: String,
    }

    let person = Person {
        name: String::from("John"),
        age: Box::new(23),
        gender: String::from("male"),
    };

    let Person {
        ref name,
        ref age,
        ref gender,
    } = person;

    println!("{:?}", person);

    let words = String::from("Hello world");
    let result = small_fn(&words);
    // words.clear(); // cannot borrow `words` as mutable more than once at a time
    println!("{}", result);
}

fn small_fn(words: &str) -> &str {
    for (i, &item) in words.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &words[..i];
        }
    }
    words
}
