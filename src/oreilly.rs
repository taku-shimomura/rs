pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_add() {
        assert_eq!(add(-100, 101), 1);
    }
}
