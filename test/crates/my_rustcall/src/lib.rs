pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn it_works() {
        let result = add(1, 2);
        assert_eq!(result, 3);
    }
}
