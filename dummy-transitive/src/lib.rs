pub const fn add(left: i32, right: i32) -> i64 {
    (right as i64) + (left as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_min() {
        assert_eq!(add(i32::MIN, i32::MIN), -4294967296);
    }

    #[test]
    fn add_max() {
        assert_eq!(add(i32::MAX, i32::MAX), 4294967294);
    }

    #[test]
    fn add_zero() {
        assert_eq!(add(0, 0), 0);
    }
}

