fn main() {
    println!("100= {}", diff_of(square_of_sums(100),sum_of_squares(100)));
}

fn sum_of_squares(n: i32) -> i32 {
  let mut sum: i32 = 0;
  for i in 1..=n {
    sum += i.pow(2);
  }
  return sum;
}

fn square_of_sums(n: i32) -> i32 {
  let mut sum: i32 = 0;
  for i in 1..=n {
    sum += i;
  }
  return sum.pow(2);
}

fn diff_of(a: i32, b: i32) -> i32 {
  return a - b;
}

#[test]
fn sum_square_of_first_two(){
  assert_eq!(sum_of_squares(2), 5);
}

#[test]
fn sum_square_of_first_ten(){
  assert_eq!(sum_of_squares(10), 385);
}

#[test]
fn square_of_sums_of_first_ten(){
  assert_eq!(square_of_sums(10), 3025);
}

#[test]
fn diff_of_sum_square_and_square_sum(){
  assert_eq!(diff_of(square_of_sums(10),sum_of_squares(10)), 2640);
}
