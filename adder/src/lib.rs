#![allow(dead_code)]
/*pub fn add(left: usize, right: usize) -> u64 {
    left + right
}*/
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
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // #[test] attribute indicates that this is a test function
    /*fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }*/
    // run ```cargo test``` to only run tests
    #[test]
    fn failure() {
        panic!("Make this test fail");
    }
    // this code panics because of failure()

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
        // assert!(foo) macro is equivalent to assert_eq!(foo, true);
    }
    // for values that should result in LHS = RHS, use assert_eq!()
    // for values that should not result in LHS = RHS, use assert_ne!()
    #[test]
    fn custom_err() {
        let false_int = false;
        assert!(false_int, "This is a custom error message");
    }
    // for tests that should panic:
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got {value}.")]
    // show expected error message as well
    fn greater_than_100() {
        Guess::new(200); // see guess struct and impl
    }
}
