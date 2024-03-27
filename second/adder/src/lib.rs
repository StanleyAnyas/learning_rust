pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greating(name: &str) -> String {
    format!("hello {name}")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}

fn length_of_last_word(s: String) -> i32 {
    let trimed_s = s.trim();
    let parts: Vec<&str> = trimed_s.split_whitespace().collect();
    let len_of_part = parts.len();
    parts[len_of_part - 1].len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("expected to fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 21,
            height: 15,
        };

        let smaller = Rectangle {
            width: 17,
            height: 10,
        };

        let _even_smaller = Rectangle {
            width: 10,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greating("Stanley");
        assert!(
            result.contains("Stanley"),
            "greeting did not contain name {result}"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // this is how to use the should_panic to test if a funtion panics in a certain context and the expected parameter is just a substring of what we expect the panic message to be
    fn check_if_between_one_and_hundred() {
        Guess::new(101);
    }

    #[test]
    fn check_if_length_of_last_word_returns_i32() {
        let string = String::from("the is a string to check the length of this hipopotamus");
        let ans = length_of_last_word(string);
        assert_eq!(ans, 11);
    }
}
