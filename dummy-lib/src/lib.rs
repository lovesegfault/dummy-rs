pub const fn times_two(num: i32) -> i64 {
    let num = num as i64;
    num.saturating_mul(2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_times_two() {
        assert_eq!(times_two(i32::MIN), -4294967296);
    }

    #[test]
    fn max_times_two() {
        assert_eq!(times_two(i32::MAX), 4294967294);
    }

    #[test]
    fn zero_times_two() {
        assert_eq!(times_two(0), 0);
    }
}

