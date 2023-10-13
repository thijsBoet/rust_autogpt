/// Function: add_five
/// 
/// # Arguments (num: u32)
/// 
/// # Returns (num: u32)
/// 
/// #Example
/// 
/// ```rust
/// let x = 100;
/// let y = add_five(x);
/// assert_eq!(y, 105);
/// ```

/**
 * This is a multiline comment
*/

// pub fn add_five(x: i32) -> i32 {
//   x + 5
// }

pub fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // fn adds_five() {
    //     let x = 100;
    //     let y = add_five(x);
    //     assert_eq!(y, 105);
    // }

    #[test]
    fn multiplies() {
        let x = 100;
        let y = multiply(x, x);
        assert_eq!(y, 10000);
    }
}
