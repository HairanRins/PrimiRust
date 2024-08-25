fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let number = 29;
    if is_prime(number) {
        println!("{} est un nombre premier.", number);
    } else {
        println!("{} n'est pas un nombre premier.", number);
    }
}
