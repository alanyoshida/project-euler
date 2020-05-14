fn main() {
    println!("the 10 001st prime number = {}", nth_prime(10001));
}


fn nth_prime(n: i64) -> i64{
  let mut count: i64 = 0;
  let mut i: i64 = 0;
  while count < n {
    i += 1;
    if is_prime(i) {
      count += 1;
      println!("{} is prime and is the {}nth prime", i, count);
    }
  }
  return i;
}

fn is_prime(n: i64) -> bool {
  if n == 1 {
    return false;
  }
	for j in 2..n {
		if n%j == 0 {
			return false;
		}
	}
	return true;
}

#[test]
fn second_prime(){
  assert_eq!(nth_prime(2), 3);
}

#[test]
fn third_prime(){
  assert_eq!(nth_prime(3), 5);
}

#[test]
fn forth_prime(){
  assert_eq!(nth_prime(4), 7);
}

#[test]
fn one_is_not_prime(){
  assert_eq!(is_prime(1), false);
}

#[test]
fn two_is_prime(){
  assert_eq!(is_prime(2), true);
}

#[test]
fn three_is_prime(){
  assert_eq!(is_prime(3), true);
}

#[test]
fn five_is_prime(){
  assert_eq!(is_prime(5), true);
}

#[test]
fn six_is_not_prime(){
  assert_eq!(is_prime(6), false);
}

#[test]
fn seven_is_prime(){
  assert_eq!(is_prime(7), true);
}

#[test]
fn four_is_not_prime(){
  assert_eq!(is_prime(4), false);
}
