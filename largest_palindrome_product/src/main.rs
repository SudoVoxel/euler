// https://stackoverflow.com/questions/199184/how-do-i-check-if-a-number-is-a-palindrome

fn main() {}
fn is_palindrome(x: i64) -> bool {
    let mut num: i64 = x;
    let mut rev: i64 = 0;
    while num > 0 {
        let dig: i64 = num % 10;
        rev *= 10;
        rev += dig;
        num /= 10;
    }
    return x == rev;
}
fn largest_palindrome_product(max_1: i64, max_2: i64) -> i64 {
    let mut max: i64 = -1;
    for x in 0..max_1 {
        for y in 0..max_2 {
            let product: i64 = x * y;
            if product > max && is_palindrome(product) {
                max = product;
            }
        }
    }
    max
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_pp_of_3_digit_nums() {
        assert_eq!(largest_palindrome_product(999, 999), 906_609);
    }
    #[test]
    fn largest_pp_of_2_digit_nums() {
        assert_eq!(largest_palindrome_product(99, 99), 8448);
    }
    #[test]
    fn largest_pp_of_4_digit_nums() {
        assert_eq!(largest_palindrome_product(9999, 9999), 98_344_389);
    }
    #[test]
    fn largest_pp_of_0() {
        assert_eq!(largest_palindrome_product(0, 0), -1);
    }
    // largest palindrome product of 1
}
