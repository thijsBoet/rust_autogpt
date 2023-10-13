// pub makes the function public and it can be called from outside the file
pub fn add_five(x: i32) -> i32 {
    x + 5
}

pub fn subract_five(x: i32) -> i32 {
    x - 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_five() {
        let x = 100;
        let y = add_five(x);
        assert_eq!(y, 105);
    }

    #[test]
    fn subtracts_five() {
        let x = 100;
        let y = subract_five(x);
        assert_eq!(y, 95);
    }
}
