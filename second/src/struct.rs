#![allow(dead_code)]
// #[derive(Debug)]
struct User {
    name: String,
    surname: String,
    number: String,
    age: i32,
    is_active: bool,
    acct_balance: f64,
}

struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn can_hold_rectangle(&self, another: &Rectangle) -> bool {
        if another.height == self.height && another.width == self.width {
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Car<'a> {
    name: &'a str,
    year: i32,
    model: &'a str,
}
struct A;
struct S(A);
struct SGen<T>(T);
fn main() {
    let stan: User = User {
        name: String::from("Stanley"),
        surname: String::from("Anyas"),
        number: String::from("+1234567789"),
        age: 22,
        is_active: true,
        acct_balance: 10_000.00,
    };

    if stan.is_active && stan.acct_balance > 0 as f64 {
        println!(
            "User {} is active and have {} acct balance",
            stan.name, stan.acct_balance
        );
    }

    let user2 = User {
        name: String::from("John"),
        surname: String::from("Doe"),
        ..stan
    };

    println!(
        "User2 {} is active and have {} acct balance",
        user2.name, user2.acct_balance
    );
    // println!("Stan phone number: {}", stan.number); // we will get an error if we run this and that is because the String number from stan was moved to user2 because type String does not implement the Copy trait so stan is no more the owner of number but user2 is
    println!("Stan's age: {}", stan.age); // this will not give an error because age being of type i32 implements the Copy trait that means the age value was just copied from stan to user2 not moved
                                          // () this is a unit

    let my_car = Car {
        name: "Ford",
        year: 2022,
        model: "Ford Focus",
    };

    println!("this is a car: {:?}", my_car);

    let result = union_types(user2.age);
    println!("{:?}", result);

    let my_rectangle = Rectangle {
        height: 100,
        width: 50,
    };

    let another_rectangle = Rectangle {
        height: 50,
        width: 100,
    };

    let ans = my_rectangle.can_hold_rectangle(&another_rectangle);
    println!("{:?}", ans);

    // let area_of_my_rectangle = area(&my_rectangle);
    // println!("The area of my rectangle is {:?}", area_of_my_rectangle);

    // println!("{:?}", my_rectangle.height);
}

fn union_types(num: i32) -> Result<i32, ()> {
    if num % 2 == 0 {
        return Ok(num);
    }
    Err(())
}

fn area(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}
