#[derive(Debug)]
struct Rectangle  {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(left: u64) -> u64 {
    add(left, 2)
}

pub fn say_hello(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_largr() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 1, height: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn greeting_contains_name() {
        let result = say_hello("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{result}`");
    }
}
