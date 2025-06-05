/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = my_cargo::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// # Examples 
/// ```
/// let x = 2;
/// let y = 4;
/// 
/// let result = my_cargo::add(x, y);
/// 
/// assert_eq!(result, 6);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
    fn it_works_add_one() {
        let arg = 5;

        let answer = add_one(arg);

        assert_eq!(answer, 6);
    }
}
