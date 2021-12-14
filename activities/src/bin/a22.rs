// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::clamp;
    #[test]
    fn test1() {
        let t1 = clamp(5, 2, 6);
        let v = 5;
        assert_eq!(t1, v, "not within guidelines")
    }

    #[test]
    fn test2() {
        let t1 = clamp(8, 2, 6);
        let v = 6;
        assert_eq!(t1, v, "not within guidelines")
    }
    use crate::div;
    #[test]
    fn test3() {
        let t3 = div(4, 2);
        let ans = Some(2);
        assert_eq!(t3, ans, " not correct division")
    }

    #[test]
    fn test4() {
        let t3 = div(9, 2);
        let ans = Some(4);
        assert_eq!(t3, ans, " not correct division")
    }

    
}
