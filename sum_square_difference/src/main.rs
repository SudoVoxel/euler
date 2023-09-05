fn main() {
    println!("{}", sum_square_diff(100));
}
fn sum_square_diff(max: u64) -> u64 {
    let sum_of_squares: u64 = (0..max + 1).into_iter().map(|x| x * x).sum();
    let sum: u64 = (0..max + 1).into_iter().sum();
    sum.pow(2) - sum_of_squares
}
