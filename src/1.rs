fn main() {
    let mut num = 0;
    let mut den = 1;
    let mut result = 0;

    while den < 100 {
        if num % den == 0 {
            result += 1;
        }
        den += 1;
    }

    println!("The number of factors of {} is {}", num, result);
}
