fn main() {}
fn even_fibonacci_numbers(n: i64) -> i64 {
    let mut one: i64 = 1;
    let mut two: i64 = 2;
    let mut sum: i64 = 0;
    while one < n {
        if one % 2 == 0 {
            sum += one;
        }
        two += one;
        one = two - one;
    }

    sum
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_4_million() {
        assert_eq!(even_fibonacci_numbers(4_000_000), 4_613_732)
    }
    #[test]
    fn sum_of_20_million() {
        assert_eq!(even_fibonacci_numbers(20_000_000), 19_544_084)
    }
    #[test]
    fn sum_of_1() {
        assert_eq!(even_fibonacci_numbers(1), 0)
    }
    #[test]
    fn sum_of_100_million() {
        assert_eq!(even_fibonacci_numbers(100_000_000), 82_790_070);
    }
}
