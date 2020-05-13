fn main() {
    println!("Largest prime factor of 600851475143 = {}", largest_prime_factor(600851475143));
}

fn is_prime(n: i64) -> bool {
	for j in 2..n {
		if n%j == 0 {
			return false;
		}
	}
	return true;
}

fn largest_prime_factor(n: i64) -> i64 {
  let (mut out, mut i ) = (0, 1);

  while i*i < n {
    if n%i == 0 && is_prime(i) {
      out = i;
    }
    i += 1;
  }
	return out;
}

#[test]
fn five_is_prime(){
  assert_eq!(is_prime(5), true);
}

#[test]
fn seven_is_prime(){
  assert_eq!(is_prime(7), true);
}

#[test]
fn four_is_not_prime(){
  assert_eq!(is_prime(4), false);
}

#[test]
fn last_prime_factor_of_13195(){
  assert_eq!(largest_prime_factor(13195), 29);
}
