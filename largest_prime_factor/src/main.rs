fn main() {}
fn largest_prime_factor(mut n: i64) -> i64 {
    // O(sqrt(n))?
    let mut factors: Vec<i64> = vec![0; 1];
    let mut d: i64 = 2;
    while n > 1 {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
        if d * d > n {
            if n > 1 {
                factors.push(n)
            }
            break;
        }
    }

    return *factors.iter().max().unwrap();
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_pf_of_1000() {
        assert_eq!(largest_prime_factor(1000), 5)
    }
    #[test]
    fn largest_pf_of_6900() {
        assert_eq!(largest_prime_factor(6900), 23)
    }
    #[test]
    fn largest_pf_of_24_million_and_1() {
        assert_eq!(largest_prime_factor(240_000_001), 268_757)
    }
    #[test]
    fn largest_pf_of_0() {
        assert_eq!(largest_prime_factor(0), 0)
    }
}
