pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_test() {
        let x = 3;

        let result = add_two(x);

        assert_eq!(result, 5);
    }
}