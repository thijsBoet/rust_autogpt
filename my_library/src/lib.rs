/// Adds left and right numbers
/// 
/// #Arguments
/// 
/// * `left` - left number
/// * `right` - right number
/// 
/// # Example
/// 
/// ```rust
/// # use my_library::add;
/// let l: usize = 20;
/// let r: usize = 30;
/// let result = add(l, r);
/// assert_eq!(result, 50);
/// ```
pub fn add(left: usize, right: usize) -> usize {
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
}
