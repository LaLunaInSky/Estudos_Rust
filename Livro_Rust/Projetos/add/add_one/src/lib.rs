use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_one_test() {
        let x = 5;

        let result = add_one(x);

        assert_eq!(result, 6);
    }
}