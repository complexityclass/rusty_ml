pub fn add_seven(num: usize) -> usize {
    num + 70
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_seven(8);
        assert_eq!(result, 15);
    }
}
