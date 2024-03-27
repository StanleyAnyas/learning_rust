#[derive(Debug)]
struct Points<T, Q> {
    x: T,
    y: Q,
    z: T,
}

impl<T, Q> Points<T, Q> {
    fn x(&self) -> &T {
        &self.x
    }
    fn _y(&self) -> &Q {
        &self.y
    }
    fn _z(&self) -> &T {
        &self.z
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = find_largest(&number_list);
    let largest2 = find_largest(&number_list2);
    let largest_char = find_largest(&char_list);

    println!("the largest number is {}", largest);
    println!("the largest2 number is {}", largest2);
    println!("the largest char is {}", largest_char);

    let my_point = Points {
        x: 32,
        y: "hello".to_string(),
        z: 50,
    };

    println!("the point is {:?} and x is {}", my_point, my_point.x());
}

fn find_largest<T: std::cmp::PartialOrd>(vec: &Vec<T>) -> &T {
    // here we use the generic type T so that we can avoid duplication of code. Now this function will be able to take a vec either an i32 or a car and find the largest
    let mut largest = &vec[0];

    for number in vec {
        if number > largest {
            largest = &number;
        }
    }
    largest
}
