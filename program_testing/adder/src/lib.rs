pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
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

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller () {
        let larger = Rectangle {
            width: 50,
            height: 50,
        };
        let smaller = Rectangle {
            width: 20,
            height: 20,
        };
        let result_true = larger.can_hold(&smaller);
        assert_eq!(result_true, true, "\nThe test didn't passed !");
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 50,
            height: 50,
        };
        let smaller = Rectangle {
            width: 20,
            height: 20,
        };
        let result_false = smaller.can_hold(&larger);
        assert_eq!(result_false, false, "\nThe test didn't passed !");
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // On attend que la partie expected soit dans le message de panic!()
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn test_will_pass () {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    #[ignore]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}

