fn main() {}

fn multiples_of_3_or_5(n: u32) -> u32 {
    (0..n)
        .into_iter()
        .filter(|x: &u32| (x % 3 == 0 || x % 5 == 0))
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multiples_of_1000() {
        assert_eq!(multiples_of_3_or_5(1000), 233168);
    }
    #[test]
    fn multiples_of_100() {
        assert_eq!(multiples_of_3_or_5(100), 2318);
    }
    #[test]
    fn multiples_of_10() {
        assert_eq!(multiples_of_3_or_5(10), 23);
    }
    #[test]
    fn multiples_of_0() {
        assert_eq!(multiples_of_3_or_5(0), 0);
    }
}
